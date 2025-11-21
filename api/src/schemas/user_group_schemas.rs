use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinPublicGroupResponse {
    pub message: String,
}
