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
use super::handlers::spot::{trade_handler, order_handler,balance_handler,kline_handler,terminated_handler,account_handler,dispatch};
use crate::utils::{self, get_env};

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

pub struct PublicStream{
    ws: Arc<Mutex<Option<BinanceStream>>>,

    trade_tx:Sender<spot::Trade>,
    trade_rx:Receiver<spot::Trade>,

    kline_tx:Sender<spot::Kline>,
    kline_rx:Receiver<spot::Kline>,

    terminated_tx:Sender<spot::Terminated>,
    terminated_rx:Receiver<spot::Terminated>,
    url: String
}

impl PublicStream {
    
    pub fn new() -> Self {
        let ws = Arc::new(Mutex::new(None));
        let (trade_tx,mut trade_rx) = mpsc::channel::<spot::Trade>(100);
        let (kline_tx,mut kline_rx) = mpsc::channel::<spot::Kline>(100);
        let (terminated_tx,mut terminated_rx) = mpsc::channel::<spot::Terminated>(100);
        
        
        let url:String = format!("{}/stream?streams=",get_env("STREAM_HOST") ).to_string();
        Self { ws,url,trade_tx, kline_tx,terminated_tx,trade_rx, kline_rx, terminated_rx}
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
        let (ws_stream, _)  = connect_async(&self.url ).await?;
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
            // ดึง WebSocketStream ออกจาก Mutex
            let ws_stream = {
                let mut guard = ws_clone.lock().await;
                guard.take().expect("WebSocket not connected")
            };

            // split เป็น write/read
            let (_, mut read) = ws_stream.split();

            while let Some(msg) = read.next().await {
                if let Ok(Message::Text(txt)) = msg {
                    dispatch_event(&txt, trade_tx.clone(), kline_tx.clone(), terminated_tx.clone()).await;
                }
            }
        });

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{timeout, Duration};
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_public_stream_listen() {
        // สร้าง stream
        let mut stream = PublicStream::new()
            .trade("BTCUSDT")
            .kline("BTCUSDT", "1m");

        // เริ่ม WebSocket stream
        stream.start_stream().await.unwrap();

        // ใช้ timeout 10 วิ เพื่อจำกัดการรัน
        let result = timeout(Duration::from_secs(10), async {
            // เริ่ม listen (dispatch_event ภายใน listen จะเรียก handler)
            stream.listen().await.unwrap();

            // รอรับ trade event จาก channel
            let mut trade_rx = stream.trade_rx;
            while let Some(trade_event) = trade_rx.recv().await {
                println!("Received trade event: {:?}", trade_event);
            }
        })
        .await;

        if result.is_err() {
            println!("Listen test timed out after 10 seconds");
        }
    }
}
