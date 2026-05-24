use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StepStatus {
    Success,
    Failed,
    Skipped,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    pub name: String,
    pub status: StepStatus,
    pub message: String,
}

impl StepResult {
    pub fn success(name: &str, message: &str) -> Self {
        Self { name: name.to_string(), status: StepStatus::Success, message: message.to_string() }
    }
    pub fn failed(name: &str, message: &str) -> Self {
        Self { name: name.to_string(), status: StepStatus::Failed, message: message.to_string() }
    }
    pub fn cancelled(name: &str) -> Self {
        Self { name: name.to_string(), status: StepStatus::Cancelled, message: String::new() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub level: String,
    pub message: String,
    pub timestamp: String,
}
