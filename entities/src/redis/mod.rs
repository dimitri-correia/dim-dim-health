use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskType {
    Email,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EmailType {
    Registration,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub task_type: TaskType,
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobEmail {
    pub email_type: EmailType,
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobEmailRegister {
    pub email: String,
    pub username: String,
    pub token: String,
}
