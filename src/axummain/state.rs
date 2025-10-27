use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::PgPool;

use crate::{
    axummain::env_loader::Settings,
    repositories::{traits::UserRepositoryTrait, user_repository::UserRepository},
};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: PgPool,

    pub user_repository: Arc<dyn UserRepositoryTrait>,
    pub jwt_secret: String,
}

impl AppState {
    pub async fn new(settings: Settings) -> Result<Self, sqlx::Error> {
        let db = PgPool::connect(&settings.database_url).await?;

        sqlx::migrate!("./migrations").run(&db).await?;

        let user_repository: Arc<dyn UserRepositoryTrait> =
            Arc::new(UserRepository::new(db.clone()));

        Ok(Self {
            db,
            user_repository,
            jwt_secret: settings.jwt_secret,
        })
    }
}
