use poise::serenity_prelude::ChannelId;
use sqlx::{Pool, Sqlite};

pub async fn subcribed_channels(conn: &Pool<Sqlite>) -> Result<Vec<ChannelId>, sqlx::Error> {
    struct SubcribedChannels {
        channel_id: i64,
    }

    Ok(
        sqlx::query_as!(SubcribedChannels, "SELECT * FROM log_channels")
            .fetch_all(conn)
            .await?
            .into_iter()
            .map(|x| unsafe { std::mem::transmute::<i64, u64>(x.channel_id) })
            .map(ChannelId::from)
            .collect(),
    )
}

pub async fn subcribe_to_channel(
    conn: &Pool<Sqlite>,
    channel: ChannelId,
) -> Result<(), sqlx::Error> {
    let channel_id = u64::from(channel);
    let channel_id: i64 = unsafe { std::mem::transmute(channel_id) };
    match sqlx::query!(
        "INSERT INTO log_channels (channel_id) VALUES (?)",
        channel_id
    )
    .execute(conn)
    .await
    {
        Ok(_) => Ok(()),
        Err(x) => {
            match x {
                sqlx::Error::Database(ref y) => {
                    if y.message() == "UNIQUE constraint failed: log_channels.channel_id" {
                        return Ok(());
                    }
                }
                _ => (),
            }
            Err(x)
        }
    }
}
