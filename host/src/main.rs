pub mod config;
pub mod db;
pub mod get_ip;
pub mod hardware_observer_client;
pub mod status;

use axum::extract::ws::WebSocket;
use axum::extract::WebSocketUpgrade;
use axum::Router;
use clokwerk::{AsyncScheduler, TimeUnits};
use config::settings;
use futures::stream::FuturesUnordered;
use futures::stream::StreamExt;
use hardware_observer_client::HardwareObserverClient;
use poise::serenity_prelude::{self as serenity, Http};
use sqlx::{Pool, Sqlite, SqlitePool};
use status::Status;
use std::fmt::Write;
use std::net::Ipv4Addr;
use std::sync::Arc;
use std::time::Duration;
use tokio::join;
use tokio::process::Command;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;

struct Data {
    db_conn: Pool<Sqlite>,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, guild_only)]
async fn reboot(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("rebooting").await?;
    Command::new(&settings().await.systemctl_path)
        .arg("start")
        .arg("systemd-reboot.service")
        .output()
        .await
        .unwrap();
    Ok(())
}

#[poise::command(slash_command, guild_only)]
async fn shutdown(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("shutting down").await?;
    Command::new(&settings().await.systemctl_path)
        .arg("start")
        .arg("systemd-poweroff.service")
        .output()
        .await
        .unwrap();
    Ok(())
}

#[poise::command(slash_command, guild_only)]
async fn status(ctx: Context<'_>) -> Result<(), Error> {
    let status = Status::new().await.to_discord_reply();
    ctx.send(status).await?;
    Ok(())
}

#[poise::command(slash_command, guild_only)]
async fn subscribe_to_logs(ctx: Context<'_>) -> Result<(), Error> {
    db::subcribe_to_channel(&ctx.data().db_conn, ctx.channel_id()).await?;
    ctx.say("done").await?;
    Ok(())
}

#[poise::command(slash_command, guild_only)]
async fn subscribed_channels(ctx: Context<'_>) -> Result<(), Error> {
    let response = db::subcribed_channels(&ctx.data().db_conn)
        .await?
        .into_iter()
        .fold(String::new(), |mut output, x| {
            let _ = write!(output, "<#{x}>");
            output
        });
    if response.is_empty() {
        ctx.say("none").await?;
    } else {
        ctx.say(response).await?;
    }
    Ok(())
}

async fn db_conn() -> Pool<Sqlite> {
    let pool = SqlitePool::connect(&settings().await.database_url)
        .await
        .unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    pool
}

async fn start_scheduler(conn: Pool<Sqlite>, http: Arc<Http>) {
    let mut scheduler = AsyncScheduler::new();
    scheduler.every(5.minutes()).run(move || {
        let conn = conn.clone();
        let http = http.clone();
        async move {
            let channels = db::subcribed_channels(&conn).await.unwrap_or_default();
            let status = Status::new().await;
            let results = channels
                .into_iter()
                .map(|x| x.send_message(&http, status.to_discord_message()))
                .collect::<FuturesUnordered<_>>()
                .collect::<Vec<_>>()
                .await
                .into_iter()
                .collect::<serenity::Result<Vec<_>>>();
            if let Err(err) = results {
                println!("err: {err:?}");
            }
        }
    });
    loop {
        scheduler.run_pending().await;
        tokio::time::sleep(Duration::from_millis(1000)).await;
    }
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // client disconnected
            return;
        };

        if socket.send(msg).await.is_err() {
            // client disconnected
            return;
        }
    }
}

#[tokio::main]
async fn main() {
    let conn = db_conn().await;
    let discord_conn = conn.clone();

    let token = &settings().await.discord_token;
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                status(),
                subscribed_channels(),
                subscribe_to_logs(),
                reboot(),
                shutdown(),
            ],
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                let allowed_guild_id = settings().await.allowed_guild;
                let ctx = ctx.clone();
                ready
                    .clone()
                    .guilds
                    .into_iter()
                    .filter(|x| x.id != allowed_guild_id)
                    .map(|x| x.id.leave(&ctx.http))
                    .collect::<FuturesUnordered<_>>()
                    .collect::<Vec<_>>()
                    .await
                    .into_iter()
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    db_conn: discord_conn,
                })
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .unwrap();
    let http = client.http.clone();
    //let hardware_observer_client = HardwareObserverClient::new();
    join!(
        async {
            client.start().await.unwrap();
        },
        /*
        async {
            hardware_observer_client.logging_run().await;
        },
        async {
            loop {
                println!("{:?}", hardware_observer_client.ping(1).await);
                tokio::time::sleep(Duration::from_secs(5)).await
            }
        },
        */
        async {
            let app = Router::new()
                .route(
                    "/ws",
                    axum::routing::any(|ws: WebSocketUpgrade| async {
                        ws.on_upgrade(handle_socket)
                    }),
                )
                .layer(
                    CorsLayer::new()
                        .allow_methods(Any)
                        .allow_origin(Any)
                        .allow_headers(Any)
                        .allow_private_network(true),
                );
            let listener = tokio::net::TcpListener::bind((
                Ipv4Addr::new(0, 0, 0, 0),
                settings().await.listen_port,
            ))
            .await
            .unwrap();
            axum::serve(listener, app).await.unwrap();
        },
        start_scheduler(conn, http)
    );
}
