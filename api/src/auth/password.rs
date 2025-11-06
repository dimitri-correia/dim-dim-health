use bcrypt::{DEFAULT_COST, hash, verify};

pub fn hash_password(password: &str, cost: Option<u32>) -> Result<String, bcrypt::BcryptError> {
    let cost = cost.unwrap_or(DEFAULT_COST);
    hash(password, cost)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_password() {
        let password = "my_secret_pwd";

        let hashed = hash_password(password, Some(4)).expect("failed to hash password");

        assert_ne!(password, hashed);

        let is_valid = verify_password(password, &hashed).expect("failed to verify password");

        assert!(is_valid);
    }

    #[test]
    fn test_verify_password_incorrect() {
        let password = "correct_pwd";
        let wrong = "wrong_pwd";

        let hashed = hash_password(password, Some(4)).expect("failed to hash password");

        let is_valid = verify_password(wrong, &hashed).expect("failed to verify password");

        assert!(!is_valid);
    }
}
