use std::sync::Arc;

use crate::{axummain::env_loader::Settings, jobs::Jobs, repositories::Repositories};
use axum::extract::FromRef;
use tracing::info;
use migration::{Migrator, MigratorTrait};
use redis::{RedisError, aio::ConnectionManager};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub redis: ConnectionManager,

    pub repositories: Arc<Repositories>,
    pub jobs: Arc<Jobs>,

    pub jwt_secret: String,
}

impl AppState {
    pub async fn create_from_settings(settings: &Settings) -> anyhow::Result<Self> {
        let db = get_db_pool(&settings.database_url).await?;
        info!("Running database migrations...");
        Migrator::up(&db, None).await?;

        let redis = get_redis_connection(&settings.redis_url).await?;

        let jwt_secret = settings.jwt_secret.clone();

        AppState::new(db, redis, jwt_secret).await
    }

    pub async fn new(
        db: DatabaseConnection,
        redis: ConnectionManager,
        jwt_secret: String,
    ) -> anyhow::Result<Self> {
        let jobs = Arc::new(Jobs::new(redis.clone()));
        let repositories = Arc::new(Repositories::new(db.clone()));

        Ok(Self {
            db,
            redis,
            repositories,
            jobs,
            jwt_secret,
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
