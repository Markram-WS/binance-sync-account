use hmac::{Hmac, Mac};
use sha2::Sha256;
use hex;
use serde_urlencoded;
use std::env;

type HmacSha256 = Hmac<Sha256>;

pub fn create_signature( params: &Vec<(&str, String)>,api_secret:&str) ->  Result<String, Box<dyn std::error::Error>> {
    let query_string  = serde_urlencoded::to_string(&params).expect("url params encode failed");
    let mut mac = HmacSha256::new_from_slice(api_secret.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(query_string.as_bytes());
    let signature_bytes = mac.finalize().into_bytes();
    let signature = hex::encode(signature_bytes);
    println!("{}",signature);
    Ok(signature)

}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    #[test]
    fn test_create_signature() {
        //use std::time::{SystemTime, UNIX_EPOCH};
        // let timestamp: String = SystemTime::now()
        //     .duration_since(UNIX_EPOCH)
        //     .expect("time went backwards")
        //     .as_millis()
        //     .to_string();
        
        let params: Vec<(&str, String)> = vec![
            ("symbol", "BTCUSDT".to_string()),
            ("side", "BUY".to_string()),
            ("type", "LIMIT".to_string()),
            ("quantity", "1".to_string()),
            ("price", "45000".to_string()),
            ("timestamp", "1760197335465".to_string()),
        ];
        let api_secret = "your_secret_key";
        assert_eq!(create_signature( &params,&api_secret).unwrap(), "eba0ec0433b2dcf16176be3df2701d7001fd9a743673d1489e9fd992513a66dc");
    }

}