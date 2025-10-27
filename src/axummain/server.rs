use crate::axummain::{router, state};

pub async fn axum_main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let app_state = state::AppState::new(&database_url, jwt_secret)
        .await
        .expect("Failed to create AppState");

    let app = router::get_main_router(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
