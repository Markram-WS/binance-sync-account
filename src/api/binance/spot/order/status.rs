use reqwest::Client;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::utils::{get_env,create_signature};


#[derive(Debug)]
pub struct Params<'a> {
    symbol :  &'a str,
    order_id :  &'a str,
    timestamp: String,
}
impl<'a> Params<'a> {
    #[allow(dead_code)]
    pub fn new(symbol:  &'a str ,order_id :  &'a str ) -> Self {
        let timestamp: String = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_millis()
        .to_string();
        Self {
            symbol,
            order_id,
            timestamp,
            
        }
        
    }
    #[allow(dead_code)]
   fn to_pairs(&self) -> Vec<(&str, String)> {
        vec![
            ("symbol", self.symbol.to_string()),
            ("orderId", self.order_id.to_string()),
            ("timestamp", self.timestamp.clone()),
        ]
    }
}

use serde::{Deserialize, Serialize};
use crate::utils::convert::{i32_to_str,i8_to_str,str_to_option_f64};
#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "orderId",deserialize_with = "i32_to_str")]
    pub order_id: String,
    #[serde(rename = "orderListId",deserialize_with = "i8_to_str")]
    pub order_list_id: String,
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    #[serde(rename = "price",deserialize_with = "str_to_option_f64", default)]
    pub price: Option<f64>,
    #[serde(rename = "origQty",deserialize_with = "str_to_option_f64", default)]
    pub orig_qty: Option<f64>,
    #[serde(rename = "executedQty",deserialize_with = "str_to_option_f64", default)]
    pub executed_qty: Option<f64>,
    #[serde(rename = "cummulativeQuoteQty",deserialize_with = "str_to_option_f64", default)]
    pub cummulative_quote_qty: Option<f64>,
    #[serde(rename = "status")]
    pub status: Option<String>,
    #[serde(rename = "timeInForce")]
    pub time_in_force: Option<String>,
    #[serde(rename = "type")]
    pub order_type: Option<String>,
    #[serde(rename = "side")]
    pub side: Option<String>,
    #[serde(rename = "stopPrice",deserialize_with = "str_to_option_f64", default)]
    pub stop_price: Option<f64>,
    #[serde(rename = "icebergQty",deserialize_with = "str_to_option_f64", default)]
    pub iceberg_qty: Option<f64>,
    #[serde(rename = "time")]
    pub time: Option<i64>,
    #[serde(rename = "updateTime")]
    pub update_time: Option<i64>,
    #[serde(rename = "isWorking")]
    pub is_working: Option<bool>,
    #[serde(rename = "workingTime")]
    pub working_time: Option<i64>,
    #[serde(rename = "origQuoteOrderQty",deserialize_with = "str_to_option_f64", default)]
    pub orig_quote_order_qty: Option<f64>,
    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: Option<String>,
}


pub async fn get_order_status<'a>(payload: Params<'a>)  -> Result< Order, Box<dyn Error>> {
    let api_host = get_env("API_HOST");
    let api_secret = get_env("API_SECRET");
    let api_key = get_env("API_KEY");
    let query_string = serde_urlencoded::to_string(&payload.to_pairs())?;
    let signature: String = create_signature(&payload.to_pairs(),&api_secret)?;
    let url = format!("{}/api/v3/order?{}&signature={}", api_host, query_string, signature);


    let client = Client::new();

    //println!("{}",&url);
    let res = client
        .get(&url)
        .header("X-MBX-APIKEY", &api_key) 
        .header("Accept", "application/json")
        .send()
        .await?;
    let status = res.status();
    let text = res.text().await?;
    println!("{}",&text);
    if status.is_success() {
        let ob: Order = serde_json::from_str(&text)?;
        Ok(ob)
    } else {
        let err = format!("status {} : {}", status.as_u16(), text);
        Err(err.into())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::env;
//     use std::sync::Once;
//     use dotenvy::dotenv;
//     static INIT: Once = Once::new();

    
//     fn init() {
//         INIT.call_once(|| {
//             dotenv().ok();
//         });
//     }

//     //get_env
//     #[tokio::test]
//     async  fn test_api_binance_spot_get_order_status(){
//         init();
//         let api_key = get_env("API_KEY_TEST");
//         let api_secret_test = get_env("API_SECRET_TEST");
//         unsafe { 
//             env::set_var("API_HOST", "https://testnet.binance.vision");
//             env::set_var("API_SECRET", api_secret_test);
//             env::set_var("API_KEY", api_key);

//         };
//         let payload = Params::new(&"BTCUSDT",&100000000i64);
        
//         println!("payload : {:?}", &payload);
//         match status(payload).await {
//             Ok(res) => {
//                 println!("response : {:?}",res);
//                 assert_eq!(200, 200);
//             },
//             Err(e) => panic!("API error: {}", e),
//         }
//     }
// }
