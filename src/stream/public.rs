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

    trade_tx:Sender<spot::Trade>,
    
    kline_tx:Sender<spot::Kline>,

    terminated_tx:Sender<spot::Terminated>,

    url: String
}

impl PublicStream {
    
    pub fn new() -> (Self,
        mpsc::Receiver<spot::Trade>,
        mpsc::Receiver<spot::Kline>,
        mpsc::Receiver<spot::Terminated>) {

        let ws = Arc::new(Mutex::new(None));
        let (trade_tx, trade_rx) = mpsc::channel::<spot::Trade>(100);
        let (kline_tx, kline_rx) = mpsc::channel::<spot::Kline>(100);
        let (terminated_tx, terminated_rx) = mpsc::channel::<spot::Terminated>(100);
        let url:String = format!("{}/stream?streams=",get_env("STREAM_HOST") ).to_string();
        ( Self { ws,url,trade_tx, kline_tx,terminated_tx}, trade_rx, kline_rx, terminated_rx)
    }

    pub fn  trade(mut self,symbol:&str) -> Self {
        self.url.push_str(&format!("{}@trade/", symbol)  );
        self
    }

    pub fn kline(mut self,symbol:&str,interval:&str) -> Self {
        self.url.push_str(&format!("{}@kline_{}/", symbol,interval) );
        self
    }

    pub async fn start_stream(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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
    
        tokio::spawn(async move {
            loop {
                let msg_opt = {
                    // lock เพื่อดึง ws จริง
                    let mut lock = ws_clone.lock().await;
                    if let Some(ws) = lock.as_mut() {
                        ws.next().await
                    } else {
                        None
                    }
                };
    
                if let Some(Ok(msg)) = msg_opt {
                    let txt = msg.to_text().unwrap();
                    dispatch_event(&txt, trade_tx.clone(), kline_tx.clone(), terminated_tx.clone()).await;
                } else {
                    // WS ยังไม่พร้อมหรือ None → รอสักพัก
                    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                }
            }
        });
    
        Ok(())
    }
    

}

#[allow(dead_code)]
async fn dispatch_event(txt: &str,trade_tx: Sender<spot::Trade>,kline_tx: Sender<spot::Kline>,terminated_tx: Sender<spot::Terminated>) {
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
            if let Ok(ev) = serde_json::from_value::<spot::Trade>(event.clone()) {
                trade_handler(ev,trade_tx).await;
            }
        }
        Some("kline") => {
            if let Ok(ev) = serde_json::from_value::<spot::Kline>(event.clone()) { 
                kline_handler(ev,kline_tx).await;
            }
        }
        Some("eventStreamTerminated") => {
            if let Ok(ev) = serde_json::from_value::<spot::Terminated>(event.clone()) {
                terminated_handler(ev,terminated_tx).await;
            }
        }
        
        _ => {}
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{timeout, Duration};
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
        let (mut stream, mut trade_rx, mut kline_rx, mut terminated_rx) = PublicStream::new();
        stream = stream.trade("BTCUSDT").kline("BTCUSDT", "1m");
        stream.start_stream().await.unwrap();
        stream.listen().await?;
        
        tokio::spawn(async move {
            while let Some(ev) = trade_rx.recv().await {
                println!("[TRADE] {:?}", ev);
            }
        });
        
        tokio::spawn(async move {
            while let Some(ev) = kline_rx.recv().await {
                println!("[KLINE] {:?}", ev);
            }
        });
        
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    
        Ok(())
    }
    
}
