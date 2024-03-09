use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Deserialize, Serialize, Debug)]
pub struct Project {
    pub id: u64,
    pub name: String,
    pub category: String,
    pub position: u64,
    pub status: u64,
    pub notes: String,
    pub creation_time: DateTime<Local>,
    pub start_time: DateTime<Local>,
    pub completion_time: DateTime<Local>,
}
