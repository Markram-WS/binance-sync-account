use reqwest::Client;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::utils::{get_env,create_signature};


#[derive(Debug)]
pub struct Params {
    timestamp: String,
}

impl Params {
    #[allow(dead_code)]
    fn new() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_millis()
            .to_string();


        Self {timestamp } 
    }
    #[allow(dead_code)]
   fn to_pairs(&self) -> Vec<(&str, String)> {
        vec![
            ("timestamp", self.timestamp.clone()),
        ]
    }
}

use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    #[serde(rename = "makerCommission")]
    pub maker_commission: u32,
    #[serde(rename = "takerCommission")]
    pub taker_commission: u32,
    #[serde(rename = "buyerCommission")]
    pub buyer_commission: u32,
    #[serde(rename = "sellerCommission")]
    pub seller_commission: u32,
    #[serde(rename = "commissionRates")]
    pub commission_rates: CommissionRates,
    #[serde(rename = "canTrade")]
    pub can_trade: bool,
    #[serde(rename = "canWithdraw")]
    pub can_withdraw: bool,
    #[serde(rename = "canDeposit")]
    pub can_deposit: bool,
    #[serde(rename = "brokered")]
    pub brokered: bool,
    #[serde(rename = "requireSelfTradePrevention")]
    pub require_self_trade_prevention: bool,
    #[serde(rename = "preventSor")]
    pub prevent_sor: bool,
    #[serde(rename = "updateTime")]
    pub update_time: u64,
    #[serde(rename = "lastUpdateId")]
    pub account_type: String,
    pub balances: Vec<Balance>,
    pub permissions: Vec<String>,
    pub uid: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommissionRates {
    pub maker: String,
    pub taker: String,
    pub buyer: String,
    pub seller: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    pub asset: String,
    pub free: String,
    pub locked: String,
}



pub async fn account_info(payload:Params) -> Result< AccountInfo, Box<dyn Error>> {
    let api_host = get_env("API_HOST");
    let api_secret = get_env("API_SECRET");
    let api_key = get_env("API_KEY");
    let query_string = serde_urlencoded::to_string(&payload.to_pairs())?;
    let signature: String = create_signature(&payload.to_pairs(),&api_secret)?;
    let url = format!("{}/api/v3/account?{}&signature={}", api_host,query_string, signature);

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
        let ob: AccountInfo = serde_json::from_str(&text)?;
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
    async  fn test_api_binance_spot_account_info(){
        init();
        let api_key = get_env("API_KEY_TEST");
        let api_secret_test = get_env("API_SECRET_TEST");
        unsafe { 
            env::set_var("API_HOST", "https://testnet.binance.vision");
            env::set_var("API_SECRET", api_secret_test);
            env::set_var("API_KEY", api_key);

        };
        let payload = Params::new();
        match account_info(payload).await {
            Ok(res) => assert_eq!(200, 200),
            Err(e) => panic!("API error: {}", e),
        }
    }
}
