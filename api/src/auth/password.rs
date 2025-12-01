use bcrypt::{DEFAULT_COST, hash, verify};

/// Hashes a password using bcrypt asynchronously.
/// Uses spawn_blocking to avoid blocking the async runtime.
pub async fn hash_password_async(
    password: String,
    cost: Option<u32>,
) -> Result<String, bcrypt::BcryptError> {
    let cost = cost.unwrap_or(DEFAULT_COST);
    tokio::task::spawn_blocking(move || hash(password, cost))
        .await
        .expect("spawn_blocking failed")
}

/// Verifies a password against a hash asynchronously.
/// Uses spawn_blocking to avoid blocking the async runtime.
pub async fn verify_password_async(
    password: String,
    password_hash: String,
) -> Result<bool, bcrypt::BcryptError> {
    tokio::task::spawn_blocking(move || verify(password, &password_hash))
        .await
        .expect("spawn_blocking failed")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hash_and_verify_password() {
        let password = "my_secret_pwd".to_string();

        let hashed = hash_password_async(password.clone(), Some(4))
            .await
            .expect("failed to hash password");

        assert_ne!(password, hashed);

        let is_valid = verify_password_async(password, hashed)
            .await
            .expect("failed to verify password");

        assert!(is_valid);
    }

    #[tokio::test]
    async fn test_verify_password_incorrect() {
        let password = "correct_pwd".to_string();
        let wrong = "wrong_pwd".to_string();

        let hashed = hash_password_async(password.clone(), Some(4))
            .await
            .expect("failed to hash password");

        let is_valid = verify_password_async(wrong, hashed)
            .await
            .expect("failed to verify password");

        assert!(!is_valid);
    }
}
