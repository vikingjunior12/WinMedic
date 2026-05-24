use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WingetEntry {
    pub id: String,
    pub name: String,
    pub current_version: String,
    pub available_version: String,
}
