use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Deserialize, Serialize, Debug)]
pub struct Project {
    pub id: u64,
    pub name: String,
    pub category: String,
    pub position: u64,
    pub status: u64,
    pub notes: Option<String>,
    pub creation_time: DateTime<Utc>,
    pub start_time: Option<DateTime<Utc>>,
    pub completion_time: Option<DateTime<Utc>>,
}
