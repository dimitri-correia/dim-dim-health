use uuid::Uuid;

/// Generates unique test data to avoid conflicts between tests
pub struct TestData {
    id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub token: String,
}

impl TestData {
    pub fn new(base_name_test: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string()[..8].to_string(),
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
}

impl Default for TestData {
    fn default() -> Self {
        Self::new("default")
    }
}
