use entities::env_loader::Settings;
use log::info;
use migration::sea_orm::{self, ConnectOptions, Database, DatabaseConnection};
use redis::{RedisError, aio::ConnectionManager};

use anyhow::Context;
use lettre::{message::Mailbox, transport::smtp::authentication::Credentials};

#[derive(Clone)]
pub struct WorkerState {
    pub db: DatabaseConnection,
    pub redis: ConnectionManager,

    pub frontend_url: String,

    pub gmail_from: Mailbox,
    pub gmail_creds: Credentials,
}

impl WorkerState {
    pub async fn create_from_settings(settings: &Settings) -> anyhow::Result<Self> {
        let db = get_db_pool(&settings.database_url).await?;
        let redis = get_redis_connection(&settings.redis_url).await?;

        WorkerState::new(
            db,
            redis,
            settings.frontend_url.clone(),
            settings.gmail_email.clone(),
            settings.gmail_password.clone(),
        )
        .await
    }

    pub async fn new(
        db: DatabaseConnection,
        redis: ConnectionManager,
        frontend_url: String,
        gmail_email: String,
        gmail_password: String,
    ) -> anyhow::Result<Self> {
        let from_str = format!("DimDim Health <{}>", gmail_email);
        let gmail_from: Mailbox = from_str
            .parse()
            .with_context(|| format!("Failed to parse from address: {}", from_str))?;

        let gmail_creds = Credentials::new(gmail_email, gmail_password);

        Ok(Self {
            db,
            redis,
            frontend_url,
            gmail_from,
            gmail_creds,
        })
    }
}

async fn get_db_pool(database_url: &str) -> Result<DatabaseConnection, sea_orm::DbErr> {
    info!("Connecting to the database at {}", database_url);
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(10)
        .min_connections(2)
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug);

    let db = Database::connect(opt).await?;
    Ok(db)
}

async fn get_redis_connection(redis_url: &str) -> Result<ConnectionManager, RedisError> {
    info!("Connecting to redis at {}", redis_url);
    let client = redis::Client::open(redis_url)?;
    client.get_connection_manager().await
}
