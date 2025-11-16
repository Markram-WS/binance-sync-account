use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::MaybeTlsStream;
// connection.rs
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures::{ StreamExt}; 
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::Receiver;
use tokio::net::TcpStream;
use tokio::sync::{mpsc,Mutex};
use serde_json::Value;
use std::sync::Arc;
type BinanceStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

use super::binance::spot;
use super::handlers::spot::{trade_handler, order_handler,balance_handler,kline_handler,terminated_handler,account_handler};
use crate::utils::{self, get_env};
#[allow(dead_code)]
pub struct PublicStream{
    ws: Arc<Mutex<Option<BinanceStream>>>,
    trade_tx: Option<mpsc::Sender<spot::Trade>>,
    kline_tx: Option<mpsc::Sender<spot::Kline>>,
    terminated_tx: Option<mpsc::Sender<spot::Terminated>>,
    url: String
}

impl PublicStream {
    
    pub fn new() -> Self {
        let ws = Arc::new(Mutex::new(None));
        let url:String = format!("{}/stream?streams=",get_env("STREAM_HOST") ).to_string();
        Self { ws,url,trade_tx: None,kline_tx: None,terminated_tx: None,}
    }

    pub fn build(mut self) -> (Self ,
    Option<mpsc::Receiver<spot::Trade>>,
    Option<mpsc::Receiver<spot::Kline>>,
    Option<mpsc::Receiver<spot::Terminated>>)
    {
        let trade_rx = if self.url.contains("@trade") {
            let (tx, rx) = mpsc::channel::<spot::Trade>(100);
            self.trade_tx = Some(tx);
            Some(rx)
        } else { None };

        let kline_rx = if self.url.contains("@kline") {
            let (tx, rx) = mpsc::channel::<spot::Kline>(100);
            self.kline_tx = Some(tx);
            Some(rx)
        } else { None };

        let terminated_rx = if self.url.contains("@terminated") {
            let (tx, rx) = mpsc::channel(100);
            self.terminated_tx = Some(tx);
            Some(rx)
        } else { None };

        (self, trade_rx, kline_rx, terminated_rx)
    }

    pub fn  trade(mut self,symbol:&str) -> Self {
        self.url.push_str(&format!("{}@trade/", symbol));
        self
    }

    pub fn kline(mut self,symbol:&str,interval:&str) -> Self {
        self.url.push_str(&format!("{}@kline_{}/", symbol,interval) );
        self
    }

    pub async fn start_stream(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}",&self.url.trim_end_matches('/').to_string() );
        let (ws_stream, _)  = connect_async(&self.url.trim_end_matches('/').to_string() ).await?;
        let mut ws = self.ws.lock().await; 
        *ws = Some(ws_stream); 
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn listen(&self) -> anyhow::Result<()> {
        let ws_clone = self.ws.clone();
        let trade_tx = self.trade_tx.clone();
        let kline_tx = self.kline_tx.clone();
        let terminated_tx = self.terminated_tx.clone();
        println!("[0]");
        tokio::spawn(async move {
            let mut guard = ws_clone.lock().await;
            let ws = guard.as_mut().expect("WS not started");
            println!("[1]");
            while let Some(msg) = ws.next().await {
                let txt = match msg {
                    Ok(m) => match m.to_text() {
                        Ok(t) => t.to_string(),
                        Err(_) => continue,
                    },
                    Err(_) => continue,
                };
                println!("[msg] {}", txt);
                println!("--------------------------");
                dispatch_event(
                    &txt,
                    &trade_tx,
                    &kline_tx,
                    &terminated_tx
                ).await;
            }
        });

    
        Ok(())
    }
    

}

#[allow(dead_code)]
async fn dispatch_event(txt: &str,
    trade_tx: &Option<Sender<spot::Trade>>,
    kline_tx: &Option<Sender<spot::Kline>>,
    terminated_tx: &Option<Sender<spot::Terminated>>) {
        
    let parsed: Value = match serde_json::from_str(txt) {
        Ok(v) => v,
        Err(e) => {
            log::warn!("JSON parse error: {:?}", e);
            return;
        }
    };

    // ✅ ตรวจว่า event อยู่ตรงไหน
    let event = if parsed.get("event").is_some() {
        &parsed["event"]
    } else {
        &parsed
    };

    match event["e"].as_str() {
        Some("trade") => {
            if let Some(tx) = trade_tx {
                if let Ok(ev) = serde_json::from_value::<spot::Trade>(event.clone()) {
                    let _ = tx.send(ev).await;
                }
            }
        }
        Some("kline") => {
            if let Some(tx) = &kline_tx {
                if let Ok(ev) = serde_json::from_value::<spot::Kline>(event.clone()) {
                    let _ = tx.send(ev).await;
                }
            }
        }
        Some("eventStreamTerminated") => {
            if let Some(tx) = &terminated_tx {
                if let Ok(ev) = serde_json::from_value::<spot::Terminated>(event.clone()) {
                    let _ = tx.send(ev).await;
                }
            }
            
        }
        
        _ => {}
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{Duration};
    use std::env;
    use std::sync::Once;
    use dotenvy::dotenv;
    
    static INIT: Once = Once::new();

    
    fn init() {
        INIT.call_once(|| {
            dotenv().ok();
        });
    }

    #[tokio::test]
    async fn test_public_stream_listen() -> anyhow::Result<()> {
        // 1️⃣ init logger / env
        init();
        unsafe { 
            env::set_var("STREAM_HOST", "wss://stream.binance.com:9443");
        };
    
        // 2️⃣ สร้าง stream + คืน Receiver สำหรับแต่ละ topic
        let (mut stream, trade_rx, kline_rx, _terminated_rx) = PublicStream::new()
        .trade("btcusdt")
        .kline("btcusdt", "1m")
        .build();
      
        stream.start_stream().await.unwrap();
        stream.listen().await?;
        
        if let Some(mut trade_rx) = trade_rx {
            tokio::spawn(async move {
                while let Some(ev) = trade_rx.recv().await {
                    println!("[TRADE] {:?}", ev);
                }
            });
        }
        
        if let Some(mut kline_rx) = kline_rx {
            tokio::spawn(async move {
                while let Some(ev) = kline_rx.recv().await {
                    println!("[KLINE] {:?}", ev);
                }
            });
        }
        
        tokio::time::sleep(Duration::from_secs(5)).await;
    
        Ok(())
    }
    
}


