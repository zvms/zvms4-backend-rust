use bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum ExportFormat {
    CSV,
    JSON,
    Excel,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExportActivityTimesOptions {
    pub start: u64, // Unix timestamp
    pub end: u64,   // Unix timestamp
    pub format: ExportFormat,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum TaskStatus {
    Pending,
    Processing,
    Done,
    Error,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Task {
    pub time: u64, // Unix timestamp
    pub actioner: ObjectId,
    pub options: ExportActivityTimesOptions,
    pub status: TaskStatus,
    pub result: Option<String>,
    pub percent: Option<f64>,
}

pub type ExportState = Mutex<HashMap<Uuid, Task>>;
