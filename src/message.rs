use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// {"id":"123","name":"test","content":"content","date":"2024-01-19T09:07:44.676424Z"}
#[derive(Serialize, Deserialize)]
pub struct Message {
    #[serde(default = "default_uuid")]
    pub id: Uuid,
    pub name: String,
    pub content: String,
    pub date: DateTime<Utc>,
}

fn default_uuid() -> Uuid {
    Uuid::new_v4()
}
