use crate::utils::convert::{str_to_f64};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "e")]
    pub event_type : String,
    #[serde(rename = "E")]
    pub event_time : i64,
    #[serde(rename = "u")]
    pub time_of_last_account_update : i64,
    #[serde(rename = "B")]
    pub balances : Vec<BalancesArray>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BalancesArray {
    #[serde(rename = "a")]
    pub asset : String,
    #[serde(rename = "f",deserialize_with = "str_to_f64")]
    pub free : f64,
    #[serde(rename = "l",deserialize_with = "str_to_f64")]
    pub locked : f64,
}
