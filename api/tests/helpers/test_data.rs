use uuid::Uuid;

/// Generates unique test data to avoid conflicts between tests
pub struct TestData {
    id: String,
}

impl TestData {
    /// Creates a new TestData instance with a unique identifier
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string()[..8].to_string(),
        }
    }

    /// Generates a unique username for testing
    pub fn username(&self, base: &str) -> String {
        format!("{}_{}", base, self.id)
    }

    /// Generates a unique email for testing
    pub fn email(&self, base: &str) -> String {
        format!("{}_{}@test.dimdim.fr", base, self.id)
    }

    /// Generates a unique token for testing
    pub fn token(&self, base: &str) -> String {
        format!("{}_{}", base, self.id)
    }
}

impl Default for TestData {
    fn default() -> Self {
        Self::new()
    }
}
