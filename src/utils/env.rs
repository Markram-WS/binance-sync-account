use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use std::env;

pub fn get_env(env_key: &str) -> String {
    std::env::var(env_key).expect(&format!("{} is missing", env_key))
   
}   

pub fn get_env_decode(env_key: &str) -> String {
    let encoded = env::var(env_key).expect(&format!("{} is missing", env_key));
    let decoded_bytes = BASE64_STANDARD
        .decode(&encoded)
        .expect(&format!("Failed to decode base64 for {}", env_key));
    String::from_utf8(decoded_bytes)
        .expect(&format!("Decoded bytes are not valid UTF-8 for {}", env_key))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    //get_env
    #[test]
    fn test_utils_get_env_existing() {
        unsafe { env::set_var("EXISING_KEY", "True") };
        assert_eq!(get_env("EXISING_KEY"), "True");
    }

    
    #[test]
    #[should_panic(expected = "`Err` value: NotPresent")]
    fn test_utils_get_env_missing() {
        unsafe {env::remove_var("MISSING_KEY")};
        assert_eq!(get_env("EXISING_KEY"), "True");
    }

    //get_env_decode
    #[test]
    fn test_utils_get_env_decode_existing() {
        unsafe { env::set_var("EXISING_KEY", "VHJ1ZQ==") };
        assert_eq!(get_env_decode("EXISING_KEY"), "True");
    }

    #[test]
    #[should_panic(expected = "`Err` value: NotPresent")]
    fn test_utils_get_env_decode_missing() {
        unsafe { env::remove_var("MISSING_KEY") };
        assert_eq!(get_env_decode("MISSING_KEY"), "True");
        
    }

    #[test]
    #[should_panic(expected = "`Err` value: InvalidByte")] 
    fn test_utils_get_env_decode_invalid_base64() {
        unsafe { env::set_var("BAD_BASE64", "not_base64!") };
        assert_eq!(get_env_decode("BAD_BASE64"), "True");
    }

}
