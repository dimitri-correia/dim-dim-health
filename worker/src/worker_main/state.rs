use log::info;
use migration::sea_orm::{self, ConnectOptions, Database, DatabaseConnection};
use redis::{RedisError, aio::ConnectionManager};

use crate::worker_main::env_loader::Settings;

#[derive(Clone)]
pub struct WorkerState {
    pub db: DatabaseConnection,
    pub redis: ConnectionManager,

    pub gmail_email: String,
    pub gmail_password: String,
}

impl WorkerState {
    pub async fn create_from_settings(settings: &Settings) -> anyhow::Result<Self> {
        let db = get_db_pool(&settings.database_url).await?;
        let redis = get_redis_connection(&settings.redis_url).await?;

        WorkerState::new(
            db,
            redis,
            settings.gmail_email.clone(),
            settings.gmail_password.clone(),
        )
        .await
    }

    pub async fn new(
        db: DatabaseConnection,
        redis: ConnectionManager,
        gmail_email: String,
        gmail_password: String,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            db,
            redis,
            gmail_email,
            gmail_password,
        })
    }
}

async fn get_db_pool(database_url: &str) -> Result<DatabaseConnection, sea_orm::DbErr> {
    info!("Connecting to the database at {}", database_url);
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(3)
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
