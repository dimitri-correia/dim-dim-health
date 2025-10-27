use axum::extract::FromRef;
use sqlx::PgPool;

use crate::repositories::user_repository::UserRepository;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: PgPool,

    pub user_repository: UserRepository,
    pub jwt_secret: String,
}

impl AppState {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let db = PgPool::connect(&database_url).await?;

        sqlx::migrate!("./migrations").run(&db).await?;

        let user_repository = UserRepository::new(db.clone());

        Ok(Self {
            db,
            user_repository,
            jwt_secret,
        })
    }
}
