use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub exp: usize,  // expiration
    pub iat: usize,  // issued_at
}

pub fn generate_token(user_id: &Uuid, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();

    let claims = Claims {
        sub: user_id.to_string(),
        exp: (now + Duration::hours(24)).timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn validate_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use uuid::Uuid;

    const SECRET: &str = "mysecretkey";

    #[test]
    fn test_generate_and_validate_token() {
        let user_id = Uuid::new_v4();
        let token = generate_token(&user_id, SECRET).expect("Failed to generate token");

        let claims = validate_token(&token, SECRET).expect("Failed to validate token");

        assert_eq!(claims.sub, user_id.to_string());

        let now = Utc::now().timestamp() as usize;
        assert!(claims.iat <= now, "Issued at should be in the past");
        assert!(claims.exp > now, "Expiration should be in the future");
    }

    #[test]
    fn test_invalid_secret() {
        let user_id = Uuid::new_v4();
        let token = generate_token(&user_id, SECRET).unwrap();

        let result = validate_token(&token, "wrongsecret");
        assert!(result.is_err(), "Validation should fail with wrong secret");
    }

    #[test]
    fn test_expired_token() {
        let user_id = Uuid::new_v4();
        let now = Utc::now();

        // Create a token with expiration in the past
        let claims = Claims {
            sub: user_id.to_string(),
            iat: (now - Duration::hours(2)).timestamp() as usize,
            exp: (now - Duration::hours(1)).timestamp() as usize,
        };

        let token = encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(SECRET.as_ref()),
        )
        .unwrap();

        let result = validate_token(&token, SECRET);
        assert!(result.is_err(), "Token should be expired");
    }
}
