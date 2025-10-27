use axum::extract::FromRef;
use sqlx::PgPool;

use crate::{axummain::env_loader::Settings, repositories::user_repository::UserRepository};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: PgPool,

    pub user_repository: UserRepository,
    pub jwt_secret: String,
}

impl AppState {
    pub async fn new(settings: Settings) -> Result<Self, sqlx::Error> {
        let db = PgPool::connect(&settings.database_url).await?;

        sqlx::migrate!("./migrations").run(&db).await?;

        let user_repository = UserRepository::new(db.clone());

        Ok(Self {
            db,
            user_repository,
            jwt_secret: settings.jwt_secret,
        })
    }
}
