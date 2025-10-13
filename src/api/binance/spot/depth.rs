use reqwest::Client;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::utils::{get_env,create_signature};

#[derive(Debug)]
pub struct Params<'a> {
    symbol: &'a str,
    limit: &'a str,
}

impl<'a> Params<'a> {
    #[allow(dead_code)]
    fn new(symbol: &'a str) -> Self {
        Self { symbol, limit: "100"  } 
    }
    #[allow(dead_code)]
    fn limit(mut self, limit: &'a str) -> Self {
        self.limit = limit;
        self
    }
    #[allow(dead_code)]
   fn to_pairs(&self) -> Vec<(&str, String)> {
        vec![
            ("symbol", self.symbol.to_string()),
            ("limit", self.limit.to_string()),
        ]
    }
}

pub async fn depth<'a>(payload:Params<'a>) -> Result<reqwest::Response , Box<dyn Error>> {
    let api_host = get_env("API_HOST");
    let api_key = get_env("API_KEY");
    let query_string = serde_urlencoded::to_string(&payload.to_pairs())?;
    let url = format!("{}/api/v3/depth?{}", api_host,query_string, );
    let client = Client::new();
    let res = client
        .get(&url)
        .header("X-MBX-APIKEY", &api_key) 
        .header("Accept", "application/json")
        .send()
        .await?;
    Ok(res)
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
    async  fn test_api_binance_spot_depth(){
        init();
        let api_key = get_env("API_KEY_TEST");
        let api_secret_test: String = get_env("API_SECRET_TEST");
        unsafe { 
            env::set_var("API_HOST", "https://testnet.binance.vision");
            env::set_var("API_SECRET", api_secret_test);
            env::set_var("API_KEY", api_key);

        };
        let payload = Params::new("BTCUSDT");
        println!("{:?}", &payload);
        let res: reqwest::Response = depth(payload).await.expect("fn error");
        let status = res.status();

        if !status.is_success() {
            let body = res.text().await.expect("Failed to read body");
            println!("{}", body);
        }

        assert_eq!(status.as_u16(), 200);
    }
}
