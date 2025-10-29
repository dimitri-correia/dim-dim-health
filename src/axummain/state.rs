use std::sync::Arc;

use axum::extract::FromRef;
use log::info;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::{axummain::env_loader::Settings, repositories::user_repository::UserRepository};
use migration::{Migrator, MigratorTrait};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: DatabaseConnection,

    pub user_repository: Arc<UserRepository>,
    pub jwt_secret: String,
}

impl AppState {
    pub async fn new(settings: &Settings) -> Result<Self, sea_orm::DbErr> {
        info!("Connecting to the database at {}", &settings.database_url);
        let db = get_db_pool(&settings.database_url).await?;

        info!("Running database migrations...");
        Migrator::up(&db, None).await?;

        let user_repository = Arc::new(UserRepository::new(db.clone()));

        Ok(Self {
            db,
            user_repository,
            jwt_secret: settings.jwt_secret.clone(),
        })
    }
}

async fn get_db_pool(database_url: &str) -> Result<DatabaseConnection, sea_orm::DbErr> {
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
        .min_connections(5)
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug);

    let db = Database::connect(opt).await?;
    Ok(db)
}
