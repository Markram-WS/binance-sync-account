use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Terminated {
    #[serde(rename = "e")]
    pub event_type : String,
    #[serde(rename = "E")]
    pub event_time : i64,
}

