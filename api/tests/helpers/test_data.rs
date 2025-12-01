use uuid::Uuid;

use crate::helpers::test_server::get_app_state;

/// Generates unique test data to avoid conflicts between tests.
///
/// The struct maintains a unique identifier that is used to generate
/// unique usernames, emails, and tokens for each test.
///
/// # Usage
///
/// ## With a base name (for simpler tests)
/// ```
/// let td = TestData::with_base_name("testcreateuser");
/// // Access directly via fields
/// let username = &td.username; // "testcreateuser_abc12345"
/// let email = &td.email;       // "testcreateuser_abc12345@test.dimdim.fr"
/// ```
///
/// ## Without a base name (for tests that need multiple unique values)
/// ```
/// let td = TestData::new();
/// let username1 = td.username("user1"); // "user1_abc12345"
/// let username2 = td.username("user2"); // "user2_abc12345"
/// ```
#[allow(dead_code)]
pub struct TestData {
    /// Unique identifier for this test data instance
    unique_id: String,
    /// Default username (generated at creation with base name)
    pub username: String,
    /// Default email (generated at creation with base name)
    pub email: String,
    /// Default password
    pub password: String,
    /// Default token (generated at creation with base name)
    pub token: String,
}

impl TestData {
    /// Create a new TestData instance with a base name.
    /// The base name is used to generate default values for username, email, and token.
    pub fn with_base_name(base_name_test: &str) -> Self {
        let unique_id = Uuid::new_v4().to_string()[..8].to_string();
        Self {
            unique_id: unique_id.clone(),
            username: format!("{}_{}", base_name_test, unique_id),
            email: format!("{}_{}@test.dimdim.fr", base_name_test, unique_id),
            password: "securepassword".to_string(),
            token: format!("{}_{}", base_name_test, unique_id),
        }
    }

    /// Create a new TestData instance without a base name.
    /// Use the helper methods to generate unique values.
    pub fn new() -> Self {
        Self::with_base_name("test")
    }

    /// Generate a unique username with the given suffix.
    /// Uses the instance's unique ID to ensure consistency within the same TestData instance.
    pub fn username(&self, suffix: &str) -> String {
        format!("{}_{}", suffix, self.unique_id)
    }

    /// Generate a unique email with the given suffix.
    /// Uses the instance's unique ID to ensure consistency within the same TestData instance.
    pub fn email(&self, suffix: &str) -> String {
        format!("{}_{}@test.dimdim.fr", suffix, self.unique_id)
    }

    /// Generate a unique token with the given suffix.
    /// Uses the instance's unique ID to ensure consistency within the same TestData instance.
    pub fn token(&self, suffix: &str) -> String {
        format!("{}_{}", suffix, self.unique_id)
    }

    /// Create a user in the database using the default username, email, and password.
    /// Returns the created user model.
    pub async fn create_user_in_db(&self) -> entities::users::Model {
        get_app_state()
            .await
            .repositories
            .user_repository
            .create(&self.username, &self.email, &self.password, false)
            .await
            .unwrap()
    }

    /// Create a user in the database with custom username, email, and password.
    /// Returns the created user model.
    #[allow(dead_code)]
    pub async fn create_custom_user_in_db(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> entities::users::Model {
        get_app_state()
            .await
            .repositories
            .user_repository
            .create(username, email, password, false)
            .await
            .unwrap()
    }

    /// Create a user in the database and generate a JWT token for them.
    /// Returns the user model and the access token.
    pub async fn create_user_with_token(&self) -> (entities::users::Model, String) {
        let user = self.create_user_in_db().await;
        let app_state = get_app_state().await;
        let token = dimdim_health_api::auth::jwt::generate_token(&user.id, &app_state.jwt_secret)
            .expect("Failed to generate JWT token");
        (user, token)
    }
}

impl Default for TestData {
    fn default() -> Self {
        Self::new()
    }
}
