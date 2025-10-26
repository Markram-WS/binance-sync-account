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
    LIMIT_MAKER,
    MARKET,
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
    price:Option<&'a f64>,
    stop_price: Option<&'a f64>,
    trailing_delta:Option<&'a f64>,
    time_in_force: Option<&'a TimeInForce>,
    timestamp: String,
}
impl<'a> Params<'a> {
    #[allow(dead_code)]
    pub fn new(symbol:  &'a str ,side :  &'a OrderSide ,quantity:&'a f64,order_type : &'a OrderTypes) -> Self {
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
                        price: None,
                        stop_price: None,
                        trailing_delta: None,
                        time_in_force: None,
                        timestamp,
                    },
            OrderTypes::LIMIT => Self {
                        symbol,
                        side,
                        order_type,
                        quantity,
                        price : None,           
                        stop_price: None,
                        trailing_delta:None,
                        time_in_force: Some(&TimeInForce::GTC,),
                        timestamp,
                    },
            OrderTypes::LIMIT_MAKER => Self {
                        symbol,
                        side,
                        order_type,
                        quantity,
                        price: None,         
                        stop_price: None,
                        trailing_delta: None,
                        time_in_force: None,
                        timestamp,
                    },
            // OrderTypes::STOP_LOSS => panic!("`Err` STOP_LOSS not rady to use"),
            // OrderTypes::STOP_LOSS_LIMIT => panic!("`Err` STOP_LOSS_LIMIT not rady to use"),
            // OrderTypes::TAKE_PROFIT => panic!("`Err` TAKE_PROFIT not rady to use"),
            // OrderTypes::TAKE_PROFIT_LIMIT => panic!("`Err` TAKE_PROFIT_LIMIT not rady to use"),
            }
        
    }
    #[allow(dead_code)]
    pub fn price(mut self, price:&'a f64) -> Self {
        self.price  =  Some(price);
        self
    }
    #[allow(dead_code)]
    pub fn stop_price(mut self, stop_price:&'a f64) -> Self {
        self.price  =  Some(stop_price);
        self
    }
    #[allow(dead_code)]
    pub fn trailing_delta(mut self, trailing_delta:&'a f64) -> Self {
        self.price  =  Some(trailing_delta);
        self
    }
    #[allow(dead_code)]
    pub fn time_in_force(mut self, time_in_force:&'a TimeInForce) -> Self {
        self.time_in_force  = Some(time_in_force);
        self
    }

    #[allow(dead_code)]
   fn to_pairs(&self) -> Vec<(&str, String)> {
        let mut pairs = Vec::new();
        pairs.push(("symbol", self.symbol.to_string()));
        pairs.push(("side", self.side.as_str().to_string()));
        pairs.push(("type", self.order_type.as_str().to_string()));
        pairs.push(("quantity", self.quantity.to_string()));
        pairs.push(("timestamp", self.timestamp.clone()));
        if let Some(v) = self.price {
            pairs.push(("price", v.to_string()));
        }
        if let Some(v) = self.stop_price {
            pairs.push(("stopPrice", v.to_string()));
        }
        if let Some(v) = self.trailing_delta {
            pairs.push(("trailingDelta", v.to_string()));
        }
        if let Some(v) = &self.time_in_force {
            pairs.push(("timeInForce", v.as_str().to_string()));
        }
        pairs
    }
}

use serde::{Deserialize, Serialize};
use crate::utils::convert::{i32_to_str,i8_to_str,str_to_option_f64};
#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub symbol: String,
    #[serde(rename = "orderId",deserialize_with = "i32_to_str")]
    pub order_id: String,
    #[serde(rename = "orderListId",deserialize_with = "i8_to_str")]
    pub order_list_id: String,
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    #[serde(rename = "transactTime")]
    pub transact_time: i64,
    #[serde(rename = "price",deserialize_with = "str_to_option_f64", default)]
    pub price: Option<f64>,
    #[serde(rename = "origQty",deserialize_with = "str_to_option_f64", default)]
    pub orig_qty: Option<f64>,
    #[serde(rename = "executedQty",deserialize_with = "str_to_option_f64", default)]
    pub executed_qty: Option<f64>,
    #[serde(rename = "origQuoteOrderQty",deserialize_with = "str_to_option_f64", default)]
    pub orig_quote_order_qty: Option<f64>,
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
    #[serde(rename = "workingTime")]
    pub working_time: Option<String>,
    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: Option<String>,
}


pub async fn create_order<'a>(payload: Params<'a>)  -> Result< Order, Box<dyn Error>> {
    let api_host = get_env("API_HOST");
    let api_secret = get_env("API_SECRET");
    let api_key = get_env("API_KEY");
    let query_string = serde_urlencoded::to_string(&payload.to_pairs())?;
    let signature: String = create_signature(&payload.to_pairs(),&api_secret)?;
    let url = format!("{}/api/v3/order?{}&signature={}", api_host, query_string, signature);


    let client = Client::new();

    println!("{}",&url);
    let res = client
        .post(&url)
        .header("X-MBX-APIKEY", &api_key) 
        .header("Accept", "application/json")
        .send()
        .await?;
    let status = res.status();
    let text = res.text().await?;
    //println!("{}",&text);
    if status.is_success() {
        let ob: Order = serde_json::from_str(&text)?;
        Ok(ob)
    } else {
        let err = format!("status {} : {}", status.as_u16(), text);
        Err(err.into())
    }
}

// use std::env;
// pub async fn test_api_binance_spot_order_create(){
//     let api_key = get_env("API_KEY_TEST");
//     let api_secret_test = get_env("API_SECRET_TEST");
//     unsafe { 
//         env::set_var("API_HOST", "https://testnet.binance.vision");
//         env::set_var("API_SECRET", api_secret_test);
//         env::set_var("API_KEY", api_key);

//     };
//     let payload = Params::new(&"BTCUSDT",&OrderSide::BUY,&0.001f64,&OrderTypes::LIMIT)
//         .time_in_force(&Some(TimeInForce::GTC))
//         .price(&Some(106000.00f64));
    
//     println!("payload : {:?}", &payload);
//     match create(payload).await {
//         Ok(res) => {
//             println!("response : {:?}",res);
//             assert_eq!(200, 200);
//         },
//         Err(e) => panic!("API error: {}", e),
//     }
// }

