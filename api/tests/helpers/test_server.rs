use async_once_cell::OnceCell;
use axum_test::TestServer;
use dimdim_health_api::axummain::{router, state};
use entities::env_loader::Settings;

static TEST_APP_STATE: OnceCell<state::AppState> = OnceCell::new();
static DB_URL: &str = "postgres://test:test-db@localhost:5433/dimdimhealthtest";
static REDIS_URL: &str = "redis://localhost:6379";

use sea_orm::{Database, DbErr};

async fn init_test_db() {
    // Check if database is already available (e.g., in CI environment)
    // If so, skip running the local script
    let db_available = match Database::connect(DB_URL).await {
        Ok(conn) => {
            let ping_result = conn.ping().await;
            ping_result.is_ok()
        }
        Err(_) => false,
    };

    if !db_available {
        // Try to run the local test DB script (for local development)
        let script_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("scripts/test-db/run_test_db.sh");

        if script_path.exists() {
            let status = tokio::process::Command::new(&script_path)
                .current_dir(
                    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                        .parent()
                        .unwrap(),
                )
                .status()
                .await
                .expect("failed to run test DB script");

            assert!(status.success(), "Test DB script failed");
        }
    }

    // wait for Postgres to actually accept SQL queries
    for _ in 0..30 {
        match Database::connect(DB_URL).await {
            Ok(conn) => {
                // force a simple query to confirm server is ready
                let _ = conn.ping().await;
                return;
            }
            Err(DbErr::Conn(_)) => {
                tokio::time::sleep(std::time::Duration::from_millis(200)).await;
            }
            Err(e) => panic!("unexpected db error: {e:?}"),
        }
    }

    panic!("DB not ready after retries");
}

pub async fn get_app_state() -> &'static state::AppState {
    TEST_APP_STATE
        .get_or_init(async {
            init_test_db().await;
            let settings = Settings {
                env: "test".to_string(),
                database_url: DB_URL.to_string(),
                redis_url: REDIS_URL.to_string(),
                frontend_url: "http://localhost:3000".to_string(),
                listenner_addr: "127.0.0.1:0".to_string(),
                env_filter: "debug".to_string(),
                jwt_secret: "test_secret".to_string(),
                number_workers: 1,
                gmail_email: "test@test.com".to_string(),
                gmail_password: "test".to_string(),
                openobserve_endpoint: "notneeded".to_string(),
            };

            state::AppState::create_from_settings(&settings)
                .await
                .unwrap()
        })
        .await
}

pub async fn get_test_server(app_state: state::AppState) -> TestServer {
    TestServer::new(router::get_main_router(app_state.clone())).unwrap()
}
