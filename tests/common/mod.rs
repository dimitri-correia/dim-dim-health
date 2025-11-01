use async_once_cell::OnceCell;
use axum_test::TestServer;
use dimdim_health::axummain::{env_loader::Settings, router, state};

static TEST_SERVER: OnceCell<TestServer> = OnceCell::new();

use sea_orm::{Database, DbErr};

async fn init_test_db() {
    let status = tokio::process::Command::new("./scripts/test-db/run_test_db.sh")
        .status()
        .await
        .expect("failed to run test DB script");

    assert!(status.success());

    // wait for Postgres to actually accept SQL queries
    let db_url = "postgres://test:test-db@localhost:5433/dimdimhealthtest";

    for _ in 0..30 {
        match Database::connect(db_url).await {
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

async fn init_test_server() -> TestServer {
    let settings = Settings {
        database_url: "postgres://test:test-db@localhost:5433/dimdimhealthtest".to_string(),
        jwt_secret: "test_secret".to_string(),
        env_filter: "debug".to_string(),
        listenner_addr: "127.0.0.1:0".to_string(),
    };
    let app_state = state::AppState::new(&settings).await.unwrap();
    TestServer::new(router::get_main_router(app_state)).unwrap()
}

pub async fn get_test_server() -> &'static TestServer {
    TEST_SERVER
        .get_or_init(async {
            init_test_db().await;
            init_test_server().await
        })
        .await
}

pub struct TestAppPaths {
    pub create_user: &'static str,
    pub current_user: &'static str,
    pub login_user: &'static str,
}

pub const APP_PATHS: TestAppPaths = TestAppPaths {
    create_user: "/api/users",
    current_user: "/api/user",
    login_user: "/api/users/login",
};
