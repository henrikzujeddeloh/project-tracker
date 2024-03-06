use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Project {
    pub id: u64,
    pub catagory: String,
    pub position: u64,
    pub status: u64,
    pub notes: String,
    pub creation_time: chrono::DateTime<chrono::Local>,
    pub start_time: chrono::DateTime<chrono::Local>,
    pub completion_time: chrono::DateTime<chrono::Local>,
}
