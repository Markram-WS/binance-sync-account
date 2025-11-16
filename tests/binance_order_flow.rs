



#[cfg(test)]
mod tests {
    use std::env;
    use std::sync::Once;
    use dotenvy::dotenv;
    use tradesys::api::binance::spot::order;
    use tradesys::api::binance::spot::depth;
    use tradesys::api::binance::spot::order::cancel;
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
    
        unsafe { 
            env::set_var("API_HOST", "https://testnet.binance.vision");
            env::set_var("API_SECRET", api_secret_test);
            env::set_var("API_KEY", api_key);

        };
        let symbol = "BTCUSDT";
        let payload_depth =  depth::Params::new(&symbol);
        let ob: depth::OrderBook = depth::get_depth(payload_depth).await.unwrap();
        let last_qty = ob.bids.last().unwrap().0;


        let payload_create_order = order::create::Params::new(
            &symbol,
            &order::OrderSide::BUY,
            &0.001 ,
            &order::OrderTypes::LimitMaker).price(&last_qty);
        println!("payload limit order : {:?}",&payload_create_order);

        let order: order::create::Order =  order::create_order(payload_create_order).await.unwrap();
        println!("create order : {:?}",&order);

        let param_status: order::status::Params<'_> = order::status::Params::new(&symbol,&order.order_id);
        let order_status:order::status::Order=  order::get_order_status(param_status).await.unwrap();
        println!("status order : {:?}",&order_status);

        let param_openorder = order::opened::Params::new(&symbol);
        let opend_orders:Vec<order::opened::Order> =  order::get_opened_order(param_openorder).await.unwrap();
        let count_orders = opend_orders.len();
        println!("count after open order : {:?}",&count_orders);

        for ord in opend_orders{
            let param_cancel: cancel::Params<'_> = order::cancel::Params::new(&symbol,&ord.order_id);
            let cancel_order:cancel::Order=  order::cancel_order(param_cancel).await.unwrap();
            println!("cancel order : {:?}",&cancel_order);
        }
        
        let param_openorder = order::opened::Params::new(&symbol);
        let opend_orders:Vec<order::opened::Order> =  order::get_opened_order(param_openorder).await.unwrap();
        let count_orders = opend_orders.len();
        println!("count after close order : {:?}",&count_orders);
        assert_eq!(&count_orders, &0);
    }
}
