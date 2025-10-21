use reqwest::Client;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::utils::{get_env,create_signature};


#[derive(Debug)]
pub struct Params<'a> {
    symbol :  &'a str,
    order_id :  &'a i64,
    timestamp: String,
}
impl<'a> Params<'a> {
    #[allow(dead_code)]
    pub fn new(symbol:  &'a str ,order_id :  &'a i64 ) -> Self {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    #[serde(rename = "symbol")]
    pub symbol: i32,
    #[serde(rename = "orderId")]
    pub order_id: i32,
    #[serde(rename = "orderListId")]
    pub order_list_id: i8,
    #[serde(rename = "clientOrderId")]
    pub client_order_id: i64,
    #[serde(rename = "price")]
    pub price: f64,
    #[serde(rename = "origQty")]
    pub orig_oty: f64,
    #[serde(rename = "executedQty")]
    pub executed_qty: f64,
    #[serde(rename = "cummulativeQuoteQty")]
    pub cummulative_quote_qty: f64,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "timeInForce")]
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub order_type: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "stopPrice")]
    pub stop_price: f64,
    #[serde(rename = "icebergQty")]
    pub iceberg_qty : f64,
    #[serde(rename = "time")]
    pub time : i64,
    #[serde(rename = "updateTime")]
    pub update_time : i64,
    #[serde(rename = "isWorking")]
    pub is_working : bool,
    #[serde(rename = "workingTime")]
    pub working_time : i64,
    #[serde(rename = "origQuoteOrderQty")]
    pub orig_quote_order_qty : String,
    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode : String,
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
