use crate::utils::convert::{str_to_f64};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    #[serde(rename = "e")]
    pub event_type : String,
    #[serde(rename = "E")]
    pub event_time : i64,
    #[serde(rename = "a")]
    pub asset : i64,
    #[serde(rename = "d",deserialize_with = "str_to_f64")]
    pub balance_delta : f64,
    #[serde(rename = "T")]
    pub clear_time : i64,
}
