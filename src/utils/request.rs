use base64::{engine::general_purpose, Engine as _};
use ed25519_dalek::SigningKey;
use ed25519_dalek::Signer;
use pkcs8::DecodePrivateKey; // <— ต้องเพิ่มบรรทัดนี้
use urlencoding::encode;

pub fn create_signature( payload: &Vec<(&str, String)>,private_key:&str) ->  Result<String, Box<dyn std::error::Error>> {
    let query_string  = serde_urlencoded::to_string(&payload).expect("url params encode failed");
    let key_bytes = general_purpose::STANDARD.decode(private_key)?;
    let signing_key = SigningKey::from_pkcs8_der(&key_bytes)
        .map_err(|e| format!("failed to parse pkcs8 ed25519 key: {}", e))?;
    let signature = signing_key.sign(query_string.as_bytes());
    let signature_b64 = general_purpose::STANDARD.encode(signature.to_bytes());
    let signature_encoded = encode(&signature_b64).into_owned();
    //println!("{}", signature_encoded);
    Ok(signature_encoded)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_utils_create_signature() {
        let params: Vec<(&str, String)> = vec![
            ("timestamp", "1760244698897".to_string()),
        ];
        let api_secret = "MC4CAQAwBQYDK2VwBCIEIHvC/UOrqjH8NrhgY4gJFkgyfB359eC9Mofmj/qFvuzB";
        assert_eq!(create_signature( &params,&api_secret).unwrap(), "qhA4ffi029s6iwNqaO9ex5qqjH7fZYVyBngZldEGm1rbXeexgwK2BK1%2F8NdviRmyq9kB1vXKCBVF4pk2C7SPBA%3D%3D");
    }

}