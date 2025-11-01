use uuid::Uuid;

pub fn generate_verification_token() -> String {
    // Generate a random UUID and convert to string without hyphens
    Uuid::new_v4().simple().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_verification_token() {
        let token = generate_verification_token();
        assert_eq!(token.len(), 32); // UUID without hyphens has length 32
    }
}
