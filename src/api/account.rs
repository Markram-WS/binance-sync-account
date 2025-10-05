use axum::http::HeaderValue;
use reqwest::Client;
use std::error::Error;
use chrono::Utc;
use lib::utils::{get_env_decode,get_env};

async fn account_info() -> Result<(), Box<dyn Error>> {
    let api_host = get_env("API_HOST");
    let api_secret = get_env("API_SECRET");
    let api_key = get_env("API_KEY");
    let url = format("{}/api/v3/account?timestamp=&signature=",api_host);

    let client = Client::new();
    let timestamp = Utc::now().timestamp_millis();
    let signature = "aaaaaa";

    let res = client
        .get(url)
        .query(&[("timestamp", timestamp), ("signature", signature)])
        .header("X-MBX-APIKEY", "xxxxx") // ใส่ API key
        .header("Accept", "application/json")
        .send()
        .await?;

    // ตรวจสอบ status
    if res.status().is_success() {
        let body = res.text().await?;
        println!("Response: {}", body);
    } else {
        eprintln!("Error: HTTP {}", res.status());
    }

    Ok(())
}

mod tests {
    use super::*;
    use std::env;
    //get_env
    #[tokio::test]
    async  fn test_api_account_info(){
        let api_secret_test = get_env_decode("API_SECRET_TEST");
        let api_key_test = get_env_decode("API_KEY_TEST");
        unsafe { 
            env::set_var("API_HOST", "https://testnet.binance.vision/api");
            env::set_var("API_SECRET_TEST", api_secret_test);
            env::set_var("API_KEY_TEST", api_key_test);
        };
        assert_eq!(1,1)

    }
}
