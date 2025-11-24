use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct ForgotPasswordRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForgotPasswordResponse {
    pub message: String,
}

#[derive(Deserialize, Validate)]
pub struct ResetPasswordRequest {
    pub token: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub new_password: String,
}

impl std::fmt::Debug for ResetPasswordRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResetPasswordRequest")
            .field("token", &self.token)
            .field("new_password", &"[REDACTED]")
            .finish()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResetPasswordResponse {
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reset_password_request_debug_redacts_password() {
        let data = ResetPasswordRequest {
            token: "test-token-123".to_string(),
            new_password: "newsupersecretpassword".to_string(),
        };

        let debug_output = format!("{:?}", data);
        
        // Password should be redacted
        assert!(debug_output.contains("[REDACTED]"));
        // Password should NOT be visible
        assert!(!debug_output.contains("newsupersecretpassword"));
        // Token should be visible
        assert!(debug_output.contains("test-token-123"));
    }
}
