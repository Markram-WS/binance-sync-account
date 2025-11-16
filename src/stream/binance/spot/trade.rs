use crate::utils::convert::{str_to_f64};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Trade {
    #[serde(rename = "e")]
    pub event_type : String,
    #[serde(rename = "E")]
    pub event_time : i64,
    #[serde(rename = "s")]
    pub symbol: String,      
    #[serde(rename = "p", deserialize_with = "str_to_f64")]
    pub price: f64,      
    #[serde(rename = "q", deserialize_with = "str_to_f64")]
    pub quantity: f64,     
    #[serde(rename = "T")]
    pub timestamp: i64,        
    #[serde(rename = "m")]
    pub is_market_maker: bool,        
    #[serde(rename = "M")]
    pub ignore: bool,         
}