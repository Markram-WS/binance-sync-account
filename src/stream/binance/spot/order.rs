
use crate::utils::convert::{str_to_f64};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c")]
    pub client_order_id: String,
    #[serde(rename = "S")]
    pub side: String,
    #[serde(rename = "o")]
    pub order_type: String,
    #[serde(rename = "f")]
    pub time_in_force: String,
    #[serde(rename = "q", deserialize_with = "str_to_f64")]
    pub quantity: f64,
    #[serde(rename = "p", deserialize_with = "str_to_f64")]
    pub price: f64,
    #[serde(rename = "P", deserialize_with = "str_to_f64")]
    pub stop_price: f64,
    #[serde(rename = "F", deserialize_with = "str_to_f64")]
    pub iceberg_quantity: f64,
    #[serde(rename = "g")]
    pub order_list_id: i64,
    #[serde(rename = "C")]
    pub orig_client_order_id: String,
    #[serde(rename = "x")]
    pub execution_type: String,
    #[serde(rename = "X")]
    pub order_status: String,
    #[serde(rename = "r")]
    pub reject_reason: String,
    #[serde(rename = "i")]
    pub order_id: i64,
    #[serde(rename = "l", deserialize_with = "str_to_f64")]
    pub last_executed_qty: f64,
    #[serde(rename = "z", deserialize_with = "str_to_f64")]
    pub cumulative_filled_qty: f64,
    #[serde(rename = "L", deserialize_with = "str_to_f64")]
    pub last_executed_price: f64,
    #[serde(rename = "n", deserialize_with = "str_to_f64")]
    pub commission_amount: f64,
    #[serde(rename = "N")]
    pub commission_asset: Option<String>,
    #[serde(rename = "T")]
    pub transaction_time: i64,
    #[serde(rename = "t")]
    pub trade_id: i64,
    #[serde(rename = "v")]
    pub prevented_match_id: Option<i64>,
    #[serde(rename = "I")]
    pub execution_id: Option<i64>,
    #[serde(rename = "w")]
    pub is_on_book: bool,
    #[serde(rename = "m")]
    pub is_maker: bool,
    #[serde(rename = "M")]
    pub ignore: bool,
    #[serde(rename = "O")]
    pub order_creation_time: i64,
    #[serde(rename = "Z", deserialize_with = "str_to_f64")]
    pub cumulative_quote_asset_qty: f64,
    #[serde(rename = "Y", deserialize_with = "str_to_f64")]
    pub last_quote_asset_qty: f64,
    #[serde(rename = "Q", deserialize_with = "str_to_f64")]
    pub quote_order_qty: f64,
    #[serde(rename = "W")]
    pub working_time: Option<i64>,
    #[serde(rename = "V")]
    pub self_trade_prevention_mode: Option<String>,
}

