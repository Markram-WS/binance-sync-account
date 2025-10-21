



#[cfg(test)]
mod tests {
    use std::env;
    use std::sync::Once;
    use dotenvy::dotenv;
    use tradesys::api::binance::spot::order;
    use tradesys::api::binance::spot::depth;
    use tradesys::utils::{get_env};
    
    static INIT: Once = Once::new();

    
    fn init() {
        INIT.call_once(|| {
            dotenv().ok();
        });
    }

    #[tokio::test]
    async  fn test_api_binance_order_flow(){
        init();
        let api_key = get_env("API_KEY_TEST");
        let api_secret_test = get_env("API_SECRET_TEST");
        println!("{}",api_key);
        unsafe { 
            env::set_var("API_HOST", "https://testnet.binance.vision");
            env::set_var("API_SECRET", api_secret_test);
            env::set_var("API_KEY", api_key);

        };
        let payload_depth =  depth::Params::new(&"BTCUSDT");
        let ob: depth::OrderBook = depth::get_depth(payload_depth).await.expect("`Err` get_depth");
        let last_qty = ob.bids.last().unwrap().1;
        println!("bid -1 : {}",last_qty);
        println!("bid 0 : {}",ob.bids[0].1);
        assert_eq!(200, 200);
        
    }
}


// pub use create::create_order;
// pub use status::get_order_status;
// pub use cancel::cancel_order;
// pub use opened::get_opened_order;



// let payload = order::create::Params::new(&"BTCUSDT");
// println!("payload : {:?}", &payload);
// match order::create_order(payload).await {
//     Ok(res) => {
//         println!("response : {:?}",res);
//         assert_eq!(200, 200);
//     },
//     Err(e) => panic!("API error: {}", e),
// }