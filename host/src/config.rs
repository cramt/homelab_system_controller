use tokio::sync::OnceCell;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub discord_token: String,
    pub database_url: String,
    pub allowed_guild: u64,
    pub systemctl_path: String,
}

pub async fn settings() -> &'static Settings {
    static ONCE: OnceCell<Settings> = OnceCell::const_new();

    ONCE.get_or_init(|| async {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap()
    })
    .await
}
