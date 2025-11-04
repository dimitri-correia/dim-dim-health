use async_once_cell::OnceCell;
use axum_test::TestServer;
use dimdim_health::axummain::{env_loader::Settings, router, state};

static TEST_APP_STATE: OnceCell<state::AppState> = OnceCell::new();
static DB_URL: &str = "postgres://test:test-db@localhost:5433/dimdimhealthtest";

use sea_orm::{Database, DbErr};

async fn init_test_db() {
    let status = tokio::process::Command::new("./scripts/test-db/run_test_db.sh")
        .status()
        .await
        .expect("failed to run test DB script");

    assert!(status.success());

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
                database_url: DB_URL.to_string(),
                jwt_secret: "test_secret".to_string(),
                env_filter: "debug".to_string(),
                listenner_addr: "127.0.0.1:0".to_string(),
            };

            state::AppState::new(&settings).await.unwrap()
        })
        .await
}

pub async fn get_test_server() -> TestServer {
    let app_state = get_app_state().await;
    TestServer::new(router::get_main_router(app_state.clone())).unwrap()
}
