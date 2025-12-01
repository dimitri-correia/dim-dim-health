use uuid::Uuid;

use crate::helpers::test_server::get_app_state;

/// Generates unique test data to avoid conflicts between tests
pub struct TestData {
    pub username: String,
    pub email: String,
    pub password: String,
    pub token: String,
}

impl TestData {
    pub fn new(base_name_test: &str) -> Self {
        Self {
            username: format!(
                "{}_{}",
                base_name_test,
                Uuid::new_v4().to_string()[..8].to_string()
            ),
            email: format!(
                "{}_{}@test.dimdim.fr",
                base_name_test,
                Uuid::new_v4().to_string()[..8].to_string()
            ),
            password: "securepassword".to_string(),
            token: format!(
                "{}_{}",
                base_name_test,
                Uuid::new_v4().to_string()[..8].to_string()
            ),
        }
    }

    pub async fn save_in_db(&self) -> entities::users::Model {
        let user = get_app_state()
            .await
            .repositories
            .user_repository
            .create(&self.username, &self.email, &self.password, false)
            .await
            .unwrap();
    }
}

impl Default for TestData {
    fn default() -> Self {
        Self::new("default")
    }
}
