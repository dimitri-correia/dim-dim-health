use std::sync::Arc;

use crate::{
    axummain::env_loader::Settings,
    repositories::{
        email_verification_repository::EmailVerificationRepository, user_repository::UserRepository,
    },
};
use axum::extract::FromRef;
use log::info;
use migration::{Migrator, MigratorTrait};
use redis::{RedisError, aio::ConnectionManager};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub redis: ConnectionManager,

    pub user_repository: Arc<UserRepository>,
    pub email_verification_repository: Arc<EmailVerificationRepository>,

    pub jwt_secret: String,
}

impl AppState {
    pub async fn new(settings: &Settings) -> anyhow::Result<Self> {
        let db = get_db_pool(&settings.database_url).await?;

        info!("Running database migrations...");
        Migrator::up(&db, None).await?;

        let user_repository = Arc::new(UserRepository::new(db.clone()));
        let email_verification_repository = Arc::new(EmailVerificationRepository::new(db.clone()));

        let redis = get_redis_connection(&settings.redis_url).await?;

        Ok(Self {
            db,
            redis,
            user_repository,
            email_verification_repository,
            jwt_secret: settings.jwt_secret.clone(),
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
