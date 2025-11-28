use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskType {
    Email,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EmailType {
    Registration,
    ResetPassword,
    EmailChange,
    MonthlyRecap,
    WeeklyRecap,
    YearlyRecap,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct JobEmailResetPassword {
    pub email: String,
    pub username: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobEmailMonthlyRecap {
    pub email: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobEmailWeeklyRecap {
    pub email: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobEmailYearlyRecap {
    pub email: String,
    pub username: String,
}

impl fmt::Display for TaskType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskType::Email => write!(f, "Email"),
        }
    }
}

impl fmt::Display for EmailType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmailType::Registration => write!(f, "Registration"),
            EmailType::ResetPassword => write!(f, "ResetPassword"),
            EmailType::EmailChange => write!(f, "EmailChange"),
            EmailType::MonthlyRecap => write!(f, "MonthlyRecap"),
            EmailType::WeeklyRecap => write!(f, "WeeklyRecap"),
            EmailType::YearlyRecap => write!(f, "YearlyRecap"),
        }
    }
}

impl fmt::Display for Job {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format the JSON data compactly. If serialization fails, fall back to Debug.
        let data_str =
            serde_json::to_string(&self.data).unwrap_or_else(|_| format!("{:?}", &self.data));
        write!(
            f,
            "Job {{ task_type: {}, data: {} }}",
            self.task_type, data_str
        )
    }
}
