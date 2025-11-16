use crate::utils::convert::{str_to_f64};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Interval {
    Seconds1,
    Minutes1,
    Minutes3,
    Minutes5,
    Minutes15,
    Minutes30,
    Hours1,
    Hours2,
    Hours4,
    Hours6,
    Hours8,
    Hours12,
    Days1,
    Days3,
    Weeks1,
    Months1,
}

impl Interval {
    pub fn as_str(&self) -> &'static str {
        match self {
            Interval::Seconds1 => "1s",
            Interval::Minutes1 => "1m",
            Interval::Minutes3 => "3m",
            Interval::Minutes5 => "5m",
            Interval::Minutes15 => "15m",
            Interval::Minutes30 => "30m",
            Interval::Hours1 => "1h",
            Interval::Hours2 => "2h",
            Interval::Hours4 => "4h",
            Interval::Hours6 => "6h",
            Interval::Hours8 => "8h",
            Interval::Hours12 => "12h",
            Interval::Days1 => "1d",
            Interval::Days3 => "3d",
            Interval::Weeks1 => "1w",
            Interval::Months1 => "1M",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Kline {
    #[serde(rename = "e")]
    pub event_type : String,
    #[serde(rename = "E")]
    pub event_time : i64,
    #[serde(rename = "s")]
    pub symbol : String,
    #[serde(rename = "k")]
    pub data : KlineDetail,

}
#[derive(Debug, Serialize, Deserialize)]
pub struct KlineDetail {
    #[serde(rename = "t")]
    pub start_time : i64,
    #[serde(rename = "T")]
    pub close_time : i64,
    #[serde(rename = "s")]
    pub symbol : String,
    #[serde(rename = "i")]
    pub interval : String,
    #[serde(rename = "f")]
    pub first_trade_id : i64,
    #[serde(rename = "L")]
    pub last_trade_id : i64,
    #[serde(rename = "o",deserialize_with = "str_to_f64")]
    pub open : f64,
    #[serde(rename = "c",deserialize_with = "str_to_f64")]
    pub close : f64,
    #[serde(rename = "h",deserialize_with = "str_to_f64")]
    pub high : f64,
    #[serde(rename = "l",deserialize_with = "str_to_f64")]
    pub low : f64,
    #[serde(rename = "v",deserialize_with = "str_to_f64")]
    pub volume : f64,
    #[serde(rename = "n")]
    pub number_of_trades : i64,
    #[serde(rename = "x")]
    pub is_kline_closed : bool,
    #[serde(rename = "q",deserialize_with = "str_to_f64")]
    pub quote_asset_volume : f64,
    #[serde(rename = "V",deserialize_with = "str_to_f64")]
    pub taker_buy_base_volume : f64,
    #[serde(rename = "Q",deserialize_with = "str_to_f64")]
    pub taker_buy_quote_volume : f64,
    #[serde(rename = "B")]
    pub ignore : String,
    


}
