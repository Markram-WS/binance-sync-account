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

pub async fn account_info(payload:Params) -> Result<reqwest::Response , Box<dyn Error>> {
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
        let res: reqwest::Response = account_info(payload).await.expect("fn error");
        let status = res.status();

        if !status.is_success() {
            let body = res.text().await.expect("Failed to read body");
            println!("{}", body);
        }

        assert_eq!(status.as_u16(), 200);
    }
}
