use std::sync::Arc;

use crate::{jobs::Jobs, repositories::Repositories, services::Services};
use axum::extract::FromRef;
use entities::env_loader::Settings;
use migration::{Migrator, MigratorTrait};
use redis::{RedisError, aio::ConnectionManager};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::{debug, error, info};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub redis: ConnectionManager,

    pub repositories: Arc<Repositories>,
    pub services: Arc<Services>,
    pub jobs: Arc<Jobs>,

    pub jwt_secret: String,
}

impl AppState {
    pub async fn create_from_settings(settings: &Settings) -> anyhow::Result<Self> {
        info!("Initializing application state...");

        let db = get_db_pool(&settings.database_url).await?;

        info!("Running database migrations...");
        match Migrator::up(&db, None).await {
            Ok(_) => info!("Database migrations completed successfully"),
            Err(e) => {
                error!(error = %e, "Database migration failed");
                return Err(e.into());
            }
        }

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
        let services = Arc::new(Services::new(db.clone()));

        debug!("Application state components initialized");

        Ok(Self {
            db,
            redis,
            repositories,
            services,
            jobs,
            jwt_secret,
        })
    }
}

async fn get_db_pool(database_url: &str) -> Result<DatabaseConnection, sea_orm::DbErr> {
    // Mask password in URL for logging
    let masked_url = mask_database_url(database_url);
    info!(url = %masked_url, "Connecting to database...");

    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(3)
        .min_connections(2)
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug);

    let db = Database::connect(opt).await?;
    info!(url = %masked_url, "Database connection established");
    Ok(db)
}

async fn get_redis_connection(redis_url: &str) -> Result<ConnectionManager, RedisError> {
    info!(url = %redis_url, "Connecting to Redis...");
    let client = redis::Client::open(redis_url)?;
    let manager = client.get_connection_manager().await?;
    info!(url = %redis_url, "Redis connection established");
    Ok(manager)
}

/// Mask password in database URL for safe logging
fn mask_database_url(url: &str) -> String {
    // Simple password masking - find :password@ pattern and replace
    // Returns original URL if parsing fails (safe fallback)
    if let Some(at_pos) = url.rfind('@') {
        // Ensure we have content before @
        if at_pos == 0 {
            return url.to_string();
        }

        if let Some(colon_pos) = url[..at_pos].rfind(':') {
            // Ensure colon is not at the start
            if colon_pos == 0 {
                return url.to_string();
            }

            if let Some(slash_pos) = url[..colon_pos].rfind('/') {
                // Validate indices before slicing
                let user_start = slash_pos + 3;
                if user_start >= colon_pos || user_start >= url.len() {
                    return url.to_string();
                }

                // Ensure the slice is valid
                if let Some(user_colon) = url.get(user_start..colon_pos).and_then(|s| s.find(':')) {
                    // Validate user slice bounds
                    let user_end = user_start + user_colon;
                    if user_end > url.len() {
                        return url.to_string();
                    }

                    if let (Some(prefix), Some(user), Some(suffix)) = (
                        url.get(..user_start),
                        url.get(user_start..user_end),
                        url.get(at_pos..),
                    ) {
                        return format!("{}{}:***{}", prefix, user, suffix);
                    }
                }
            }
        }
    }
    url.to_string()
}
