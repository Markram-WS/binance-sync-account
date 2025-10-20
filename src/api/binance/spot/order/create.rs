use reqwest::Client;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::utils::{get_env,create_signature};

#[derive(Debug)]
pub enum OrderSide {
    BUY,
    SELL,
}
impl OrderSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderSide::BUY => "BUY",
            OrderSide::SELL => "SELL",
        }
    }
}
#[derive(Debug)]
pub enum OrderTypes {
    LIMIT,
    MARKET,
    LIMIT_MAKER,
    // STOP_LOSS,
    // STOP_LOSS_LIMIT,
    // TAKE_PROFIT,
    // TAKE_PROFIT_LIMIT,
    
}
impl OrderTypes {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderTypes::LIMIT => "LIMIT",
            OrderTypes::LIMIT_MAKER => "LIMIT_MAKER",
            OrderTypes::MARKET => "MARKET",
            // OrderTypes::STOP_LOSS => "STOP_LOSS",
            // OrderTypes::STOP_LOSS_LIMIT => "STOP_LOSS_LIMIT",
            // OrderTypes::TAKE_PROFIT => "TAKE_PROFIT",
            // OrderTypes::TAKE_PROFIT_LIMIT => "TAKE_PROFIT_LIMIT",
            
        }
    }
}

#[derive(Debug)]
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
}
impl TimeInForce {
    pub fn as_str(&self) -> &'static str {
        match self {
            TimeInForce::GTC => "GTC",
            TimeInForce::IOC => "IOC",
            TimeInForce::FOK => "FOK",
        }
    }
}

#[derive(Debug)]
pub struct Params<'a> {
    symbol :  &'a str,
    side :  &'a OrderSide,
    order_type : &'a OrderTypes ,
    quantity:&'a f64,
    price:&'a Option<f64>,
    stop_price:&'a Option<f64>,
    trailing_delta:&'a Option<f64>,
    time_in_force: &'a Option<TimeInForce>,
    timestamp: String,
}
impl<'a> Params<'a> {
    #[allow(dead_code)]
    fn new(symbol:  &'a str ,side :  &'a OrderSide ,quantity:&'a f64,order_type : &'a OrderTypes) -> Self {
        let timestamp: String = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_millis()
        .to_string();
        match &order_type {
            OrderTypes::MARKET => Self {
                        symbol,
                        side,
                        order_type,
                        quantity,
                        price: &None,
                        stop_price: &None,
                        trailing_delta: &None,
                        time_in_force: &None,
                        timestamp,
                    },
            OrderTypes::LIMIT => Self {
                        symbol,
                        side,
                        order_type,
                        quantity,
                        price: &Some(0.0),           // set later
                        stop_price: &None,
                        trailing_delta: &None,
                        time_in_force: &Some(TimeInForce::GTC), // default
                        timestamp,
                    },
            OrderTypes::LIMIT_MAKER => Self {
                        symbol,
                        side,
                        order_type,
                        quantity,
                        price: &Some(0.0),           // set later
                        stop_price: &None,
                        trailing_delta: &None,
                        time_in_force: &Some(TimeInForce::GTC), // always GTC
                        timestamp,
                    },
            // OrderTypes::STOP_LOSS => panic!("`Err` STOP_LOSS not rady to use"),
            // OrderTypes::STOP_LOSS_LIMIT => panic!("`Err` STOP_LOSS_LIMIT not rady to use"),
            // OrderTypes::TAKE_PROFIT => panic!("`Err` TAKE_PROFIT not rady to use"),
            // OrderTypes::TAKE_PROFIT_LIMIT => panic!("`Err` TAKE_PROFIT_LIMIT not rady to use"),
            }
        
    }
    #[allow(dead_code)]
    fn price(mut self, price:&'a Option<f64>) -> Self {
        self.price  = price;
        self
    }
    #[allow(dead_code)]
    fn time_in_force(mut self, time_in_force:&'a Option<TimeInForce>) -> Self {
        self.time_in_force  = time_in_force;
        self
    }

    #[allow(dead_code)]
   fn to_pairs(&self) -> Vec<(&str, String)> {
        vec![
            ("symbol", self.symbol.to_string()),
            ("side", self.side.as_str().to_string()),
            ("type", self.order_type.as_str().to_string()),
            ("quantity", self.quantity.to_string()),
            ("price", self.price.map_or(String::new(), |v| v.to_string())),
            ("stopPrice", self.stop_price.map_or(String::new(), |v| v.to_string())),
            ("trailingDelta", self.trailing_delta.map_or(String::new(), |v| v.to_string())),
            ("timeInForce", self.time_in_force.as_ref().map_or(String::new(), |v| v.as_str().to_string())),
            ("timestamp", self.timestamp.clone()),
        ]
    }
}

use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub symbol: String,
    #[serde(rename = "orderId")]
    pub order_id: i32,
    #[serde(rename = "orderListId")]
    pub order_list_id: i32,
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    #[serde(rename = "transactTime")]
    pub transact_time: i64,
    pub price: f64,
    #[serde(rename = "origQty")]
    pub orig_oty: f64,
    #[serde(rename = "executedQty")]
    pub executed_qty: f64,
    #[serde(rename = "origQuoteOrderQty")]
    pub orig_quote_order_qty: f64,
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
    #[serde(rename = "workingTime")]
    pub working_time: String,
    #[serde(rename = "selfTradePreventionMode")]
    pub self_tade_prevention_mode : String
}


pub async fn create<'a>(payload: Params<'a>)  -> Result< Order, Box<dyn Error>> {
    let api_host = get_env("API_HOST");
    let api_secret = get_env("API_SECRET");
    let api_key = get_env("API_KEY");
    let query_string = serde_urlencoded::to_string(&payload.to_pairs())?;
    let signature: String = create_signature(&payload.to_pairs(),&api_secret)?;
    let url = format!("{}/api/v3/order/test?{}&signature={}", api_host, query_string, signature);


    let client = Client::new();

    //println!("{}",&url);
    let res = client
        .post(&url)
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
        let err = format!("API error {}: {}", status.as_u16(), status.as_str());
        Err(err.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::Once;
    use dotenvy::dotenv;
    static INIT: Once = Once::new();

    
    fn init() {
        INIT.call_once(|| {
            dotenv().ok();
        });
    }

    //get_env
    #[tokio::test]
    async  fn test_api_binance_spot_order_create(){
        init();
        let api_key = get_env("API_KEY_TEST");
        let api_secret_test = get_env("API_SECRET_TEST");
        unsafe { 
            env::set_var("API_HOST", "https://testnet.binance.vision");
            env::set_var("API_SECRET", api_secret_test);
            env::set_var("API_KEY", api_key);

        };
        let payload = Params::new(&"BTCUSDT",&OrderSide::BUY,&0.001f64,&OrderTypes::LIMIT)
            .time_in_force(&Some(TimeInForce::GTC))
            .price(&Some(106000.00f64));
        
        println!("payload : {:?}", &payload);
        match create(payload).await {
            Ok(res) => {
                println!("response : {:?}",res);
                assert_eq!(200, 200);
            },
            Err(e) => panic!("API error: {}", e),
        }
    }
}
