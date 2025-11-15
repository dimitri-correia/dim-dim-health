/// Test data generation utilities to ensure unique data across test runs
/// 
/// This module provides helper functions for generating unique test data
/// that won't conflict even if tests are run multiple times or concurrently.

use uuid::Uuid;

/// Generate a unique username for testing
/// 
/// Creates a username with a UUID suffix to ensure uniqueness across test runs.
/// 
/// # Arguments
/// * `prefix` - Optional prefix for the username. Defaults to "testuser"
/// 
/// # Example
/// ```
/// use tests_helpers::test_data::unique_username;
/// 
/// let username = unique_username(None);
/// assert!(username.starts_with("testuser_"));
/// 
/// let custom_username = unique_username(Some("admin"));
/// assert!(custom_username.starts_with("admin_"));
/// ```
pub fn unique_username(prefix: Option<&str>) -> String {
    let prefix = prefix.unwrap_or("testuser");
    format!("{}_{}", prefix, Uuid::new_v4().simple())
}

/// Generate a unique email for testing
/// 
/// Creates an email address with a UUID to ensure uniqueness across test runs.
/// 
/// # Arguments
/// * `username` - Optional username part. If None, generates a unique username
/// * `domain` - Optional domain part. Defaults to "test.example.com"
/// 
/// # Example
/// ```
/// use tests_helpers::test_data::unique_email;
/// 
/// let email = unique_email(None, None);
/// assert!(email.contains("@test.example.com"));
/// 
/// let custom_email = unique_email(Some("admin"), Some("company.com"));
/// assert!(custom_email.starts_with("admin_"));
/// assert!(custom_email.ends_with("@company.com"));
/// ```
pub fn unique_email(username: Option<&str>, domain: Option<&str>) -> String {
    let domain = domain.unwrap_or("test.example.com");
    match username {
        Some(user) => format!("{}_{}@{}", user, Uuid::new_v4().simple(), domain),
        None => format!("user_{}@{}", Uuid::new_v4().simple(), domain),
    }
}

/// Generate a unique token/string for testing
/// 
/// Creates a unique string that can be used for tokens, codes, or other identifiers.
/// 
/// # Arguments
/// * `prefix` - Optional prefix for the token. Defaults to "token"
/// 
/// # Example
/// ```
/// use tests_helpers::test_data::unique_token;
/// 
/// let token = unique_token(None);
/// assert!(token.starts_with("token_"));
/// 
/// let custom_token = unique_token(Some("verification"));
/// assert!(custom_token.starts_with("verification_"));
/// ```
pub fn unique_token(prefix: Option<&str>) -> String {
    let prefix = prefix.unwrap_or("token");
    format!("{}_{}", prefix, Uuid::new_v4().simple())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_username_default() {
        let username1 = unique_username(None);
        let username2 = unique_username(None);
        
        assert!(username1.starts_with("testuser_"));
        assert!(username2.starts_with("testuser_"));
        assert_ne!(username1, username2);
    }

    #[test]
    fn test_unique_username_custom_prefix() {
        let username = unique_username(Some("admin"));
        assert!(username.starts_with("admin_"));
    }

    #[test]
    fn test_unique_email_default() {
        let email1 = unique_email(None, None);
        let email2 = unique_email(None, None);
        
        assert!(email1.contains("@test.example.com"));
        assert!(email2.contains("@test.example.com"));
        assert_ne!(email1, email2);
    }

    #[test]
    fn test_unique_email_custom() {
        let email = unique_email(Some("admin"), Some("company.com"));
        assert!(email.starts_with("admin_"));
        assert!(email.ends_with("@company.com"));
    }

    #[test]
    fn test_unique_token_default() {
        let token1 = unique_token(None);
        let token2 = unique_token(None);
        
        assert!(token1.starts_with("token_"));
        assert!(token2.starts_with("token_"));
        assert_ne!(token1, token2);
    }

    #[test]
    fn test_unique_token_custom_prefix() {
        let token = unique_token(Some("verification"));
        assert!(token.starts_with("verification_"));
    }
}
