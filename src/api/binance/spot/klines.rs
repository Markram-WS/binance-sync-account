
use crate::utils::{get_env};

#[derive(Debug)]
pub enum Interval {
    Seconds1,
    Minutes1,
    Minutes3,
    Minutes5,
    Minutes15,
    Minutes30,
    Hours1,
    Hours2,
    Hours4,
    Hours6,
    Hours8,
    Hours12,
    Days1,
    Days3,
    Weeks1,
    Months1,
}

impl Interval {
    pub fn as_str(&self) -> &'static str {
        match self {
            Interval::Seconds1 => "1s",
            Interval::Minutes1 => "1m",
            Interval::Minutes3 => "3m",
            Interval::Minutes5 => "5m",
            Interval::Minutes15 => "15m",
            Interval::Minutes30 => "30m",
            Interval::Hours1 => "1h",
            Interval::Hours2 => "2h",
            Interval::Hours4 => "4h",
            Interval::Hours6 => "6h",
            Interval::Hours8 => "8h",
            Interval::Hours12 => "12h",
            Interval::Days1 => "1d",
            Interval::Days3 => "3d",
            Interval::Weeks1 => "1w",
            Interval::Months1 => "1M",
        }
    }
}

#[derive(Debug)]
pub struct Params<'a> {
    symbol: &'a str,
    interval: &'a Interval,
    start_time: &'a i64,
    end_time: &'a i64,
    time_zone: &'a str,
    limit: &'a str,
    
}

impl<'a> Params<'a> {
    #[allow(dead_code)]
    fn new(symbol: &'a str,interval: &'a Interval,start_time: &'a i64,end_time: &'a i64) -> Self {
        Self { symbol,interval, start_time, end_time,time_zone:"0", limit: "100"  } 
    }
    #[allow(dead_code)]
    fn time_zone(mut self, time_zone: &'a str) -> Self {
        self.time_zone = time_zone;
        self
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
            ("interval", self.interval.as_str().to_string()),
            ("startTime", self.start_time.to_string()),
            ("endTime", self.end_time.to_string()),
            ("timeZone", self.time_zone.to_string()),
            ("limit", self.limit.to_string()),
        ]
    }
}

#[derive(Debug)]
pub struct Kline {
    pub open_time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub close_time: i64,
    pub quote_asset_volume: f64,
    pub number_of_trades: u64,
    pub taker_buy_base_volume: f64,
    pub taker_buy_quote_volume: f64,
    pub ignore: f32,
}

use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct RawKline(
    i64,     // open time
    String,  // open
    String,  // high
    String,  // low
    String,  // close
    String,  // volume
    i64,     // close time
    String,  // quote asset volume
    u64,     // number of trades
    String,  // taker buy base volume
    String,  // taker buy quote volume
    String,  // ignore
);
impl From<RawKline> for Kline {
    fn from(raw: RawKline) -> Self {
        Kline {
            open_time: raw.0,
            open: raw.1.parse().unwrap_or(0.0),
            high: raw.2.parse().unwrap_or(0.0),
            low: raw.3.parse().unwrap_or(0.0),
            close: raw.4.parse().unwrap_or(0.0),
            volume: raw.5.parse().unwrap_or(0.0),
            close_time: raw.6,
            quote_asset_volume: raw.7.parse().unwrap_or(0.0),
            number_of_trades: raw.8,
            taker_buy_base_volume: raw.9.parse().unwrap_or(0.0),
            taker_buy_quote_volume: raw.10.parse().unwrap_or(0.0),
            ignore : raw.11.parse().unwrap_or(0.0 ),
        }
    }
}
pub async fn klines<'a>(payload: Params<'a>) -> Result<Vec<Kline>, Box<dyn std::error::Error>> {
    let api_host = get_env("API_HOST");
    let api_key = get_env("API_KEY");
    let query_string = serde_urlencoded::to_string(&payload.to_pairs())?;
    let url = format!("{}/api/v3/klines?{}", api_host, query_string);
    let client = reqwest::Client::new();

    let res = client
        .get(&url)
        .header("X-MBX-APIKEY", &api_key)
        .header("Accept", "application/json")
        .send()
        .await?;

    let status = res.status();
    let text = res.text().await?;
    //println!("{} : {}", status.as_u16(), status.as_str());

    if status.is_success() {
        let raw_klines: Vec<RawKline> = serde_json::from_str(&text)?;
        let klines: Vec<Kline> = raw_klines.into_iter().map(Kline::from).collect();
        Ok(klines)
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
    async  fn test_api_binance_spot_kline(){
        init();
        let api_key = get_env("API_KEY_TEST");
        let api_secret_test: String = get_env("API_SECRET_TEST");
        unsafe { 
            env::set_var("API_HOST", "https://testnet.binance.vision");
            env::set_var("API_SECRET", api_secret_test);
            env::set_var("API_KEY", api_key);

        };
        let payload = Params::new("BTCUSDT",&Interval::Days1,&1759251600000i64,&1760029200000i64);
        println!("payload : {:?}", &payload);
        match klines(payload).await {
            Ok(res) => {
                println!("response : {:?}",res);
                assert_eq!(200, 200);
            }
            Err(e) => panic!("API error: {}", e),
        }
        

    }
}
