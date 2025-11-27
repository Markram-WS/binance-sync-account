use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::MaybeTlsStream;
// connection.rs
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures::{ StreamExt}; 
use tokio::net::TcpStream;
use tokio::sync::{mpsc,Mutex};
use serde_json::Value;
use std::sync::Arc;
type BinanceStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

use super::spot;
use crate::utils::{get_env};

pub enum Event {
    Trade(spot::Trade),
    Kline(spot::Kline),
    Ticker(spot::Ticker),
}

#[allow(dead_code)]
pub struct PublicStream{
    ws: Arc<Mutex<Option<BinanceStream>>>,
    event_tx: mpsc::Sender<Event>,
    url: String
}

impl PublicStream {
    
    pub fn new() ->  (Self,mpsc::Receiver<Event>) {
        let ws = Arc::new(Mutex::new(None));
        let url:String = format!("{}/stream?streams=",get_env("STREAM_HOST") ).to_string();
        let (tx,  rx) = mpsc::channel::<Event>(100);
        (Self { ws,url,event_tx:tx},rx)
    }
    pub fn  trade(&mut self,symbol:&str)   {
        self.url.push_str(&format!("{}@trade/", symbol));
    
    }

    pub fn kline(&mut self,symbol:&str,interval:&str)    {
        self.url.push_str(&format!("{}@kline_{}/", symbol,interval) );
   
    }

    pub fn ticker(&mut self,symbol:&str)    {
        self.url.push_str(&format!("{}@ticker/", symbol) );
   
    }

    pub async fn start_stream(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}",&self.url.trim_end_matches('/').to_string() );
        let (ws_stream, _)  = connect_async(&self.url.trim_end_matches('/').to_string() ).await?;
        let mut ws = self.ws.lock().await; 
        *ws = Some(ws_stream); 
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn listen(&self) -> anyhow::Result<()> {
        let ws_clone = self.ws.clone();
        let event_tx = self.event_tx.clone();
        tokio::spawn(async move {
            let mut guard = ws_clone.lock().await;
            let ws = guard.as_mut().expect("WS not started");
            while let Some(msg) = ws.next().await {
                let txt = match msg {
                    Ok(m) => match m.to_text() {
                        Ok(t) => t.to_string(),
                        Err(_) => continue,
                    },
                    Err(_) => continue,
                };
                dispatch_event(
                    &txt,&event_tx
                ).await;
            }
        });

    
        Ok(())
    }
    

}

#[allow(dead_code)]
async fn dispatch_event(txt: &str,event_tx: &mpsc::Sender<Event>) {
        
    let parsed: Value = match serde_json::from_str(txt) {
        Ok(v) => v,
        Err(e) => {
            log::warn!("JSON parse error: {:?}", e);
            return;
        }
    };

    // ✅ ตรวจว่า event อยู่ตรงไหน
    let event_type = parsed.get("e")
    .or_else(|| parsed.get("data").and_then(|d| d.get("e")))
    .and_then(|v| v.as_str());

    match event_type {
        Some("trade") => {
            if let Some(data) = parsed.get("data") {
                match serde_json::from_value::<spot::Trade>(data.clone()) {
                    Ok(ev) => {
                        
                            let _ = event_tx.send(Event::Trade(ev)).await;  
                        
                    }
                    Err(e) => {
                        println!("Failed to parse trade event: {:?}", e);
                    }
                }
            }
        }

        Some("kline") => {
            if let Some(data) = parsed.get("data") {
                match serde_json::from_value::<spot::Kline>(data.clone()) {
                    Ok(ev) => {
                       
                            let _ = event_tx.send(Event::Kline(ev)).await;  
                        
                    }
                    Err(e) => {
                        println!("Failed to parse trade event: {:?}", e);
                    }
                }
            }
        }
        Some("24hrTicker") => {
            if let Some(data) = parsed.get("data") {
                match serde_json::from_value::<spot::Ticker>(data.clone()) {
                    Ok(ev) => {
                       
                            let _ = event_tx.send(Event::Ticker(ev)).await;  
                        
                    }
                    Err(e) => {
                        println!("Failed to parse trade event: {:?}", e);
                    }
                }
            }
        }
        _ => {}
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::Once;
    use dotenvy::dotenv;
    use tokio::time::{timeout, Duration};
    static INIT: Once = Once::new();

    
    fn init() {
        INIT.call_once(|| {
            dotenv().ok();
        });
    }

    #[tokio::test]
    async fn test_public_stream_listen()  {
        // 1️⃣ init logger / env
        init();
        unsafe { 
            env::set_var("STREAM_HOST", "wss://stream.binance.com:9443");
        };
    
        // 2️⃣ สร้าง stream + คืน Receiver สำหรับแต่ละ topic
        let (mut  stream_trade, mut trade_rx) = PublicStream::new();
        stream_trade.trade("btcusdt");

        let (mut  stream_kline, mut kline_rx) = PublicStream::new();
        stream_kline.kline("btcusdt", "1m");
   
        
        stream_trade.start_stream().await.unwrap();
        stream_kline.start_stream().await.unwrap();
        stream_trade.listen().await.unwrap();
        stream_kline.listen().await.unwrap();
        let result = timeout(Duration::from_secs(3), async {
            loop {
                tokio::select! {
                    Some(Event::Trade(t)) = trade_rx.recv() => {
                        println!("TRADE: {:?}", t);
                        break ;  
                    },
                    Some(Event::Kline(k)) = kline_rx.recv() => {
                        println!("KLINE: {:?}", k);
                        break ;
                    },
                    else => break,
                }
            }
        }).await;

        assert!(result.is_ok(), "Did not receive event in time");
    }
    
}


