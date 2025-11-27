use serde::{Deserialize, Serialize};
use crate::utils::convert::str_to_f64;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ticker {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: i64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "p", deserialize_with = "str_to_f64")]
    pub price_change: f64,

    #[serde(rename = "P", deserialize_with = "str_to_f64")]
    pub price_change_percent: f64,

    #[serde(rename = "w", deserialize_with = "str_to_f64")]
    pub weighted_avg_price: f64,

    #[serde(rename = "x", deserialize_with = "str_to_f64")]
    pub first_trade_before_window: f64,

    #[serde(rename = "c", deserialize_with = "str_to_f64")]
    pub last_price: f64,

    #[serde(rename = "Q", deserialize_with = "str_to_f64")]
    pub last_quantity: f64,

    #[serde(rename = "b", deserialize_with = "str_to_f64")]
    pub best_bid_price: f64,

    #[serde(rename = "B", deserialize_with = "str_to_f64")]
    pub best_bid_quantity: f64,

    #[serde(rename = "a", deserialize_with = "str_to_f64")]
    pub best_ask_price: f64,

    #[serde(rename = "A", deserialize_with = "str_to_f64")]
    pub best_ask_quantity: f64,

    #[serde(rename = "o", deserialize_with = "str_to_f64")]
    pub open_price: f64,

    #[serde(rename = "h", deserialize_with = "str_to_f64")]
    pub high_price: f64,

    #[serde(rename = "l", deserialize_with = "str_to_f64")]
    pub low_price: f64,

    #[serde(rename = "v", deserialize_with = "str_to_f64")]
    pub volume: f64,

    #[serde(rename = "q", deserialize_with = "str_to_f64")]
    pub quote_volume: f64,

    #[serde(rename = "O")]
    pub open_time: i64,

    #[serde(rename = "C")]
    pub close_time: i64,

    #[serde(rename = "F")]
    pub first_trade_id: i64,

    #[serde(rename = "L")]
    pub last_trade_id: i64,

    #[serde(rename = "n")]
    pub total_trades: i64,
}
