use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::MaybeTlsStream;
// connection.rs
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures::{ StreamExt}; 
use super::handlers::spot::{trade_handler, order_handler,balance_handler,kline_handler,terminated_handler,account_handler};
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::Receiver;
use tokio::net::TcpStream;
use tokio::sync::{mpsc,Mutex};
use super::binance::spot;
use serde_json::Value;
use std::sync::Arc;
type BinanceStream = WebSocketStream<MaybeTlsStream<TcpStream>>;



pub struct UserDataStream{
    ws: Arc<Mutex<Option<BinanceStream>>>,

    account_tx:Sender<spot::Account>,
    account_rx:Receiver<spot::Account>,

    order_tx:Sender<spot::Order>,
    order_rx:Receiver<spot::Order>,

    balance_tx:Sender<spot::Balance>,
    balance_rx:Receiver<spot::Balance>,

    terminated_tx:Sender<spot::Terminated>,
    terminated_rx:Receiver<spot::Terminated>,
    url: String
}

impl UserDataStream {
    
    pub fn new() -> Self {
        let ws = Arc::new(Mutex::new(None));

        let (account_tx,mut account_rx) = mpsc::channel::<spot::Account>(100);
        let (order_tx,mut order_rx) = mpsc::channel::<spot::Order>(100);
        let (balance_tx,mut balance_rx) = mpsc::channel::<spot::Balance>(100);
        let (terminated_tx,mut terminated_rx) = mpsc::channel::<spot::Terminated>(100);
        let url:String = "wss://stream.binance.com:9443/stream?streams=".to_string();
        Self { ws,url, order_tx ,account_tx,balance_tx,terminated_tx,order_rx ,account_rx,balance_rx,terminated_rx}
    }

    pub async fn trade(&mut self,symbol:&str) {
        self.url.push_str(&format!("{}@trade/", symbol)  );
    }

    pub async fn kline(&mut self,symbol:&str,interval:&str) {
        self.url.push_str(&format!("{}@kline_{}/", symbol,interval) );
    }

    pub async fn start_stream(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let (ws_stream, _)  = connect_async(&self.url ).await?;
        let mut ws = self.ws.lock().await; 
        *ws = Some(ws_stream); 
        Ok(())
    }


    async fn listen(&self) -> anyhow::Result<()> {
        let ws_clone = self.ws.clone();
    
        tokio::spawn(async move {
            let ws_stream = {
                let mut guard = ws_clone.lock().await;
    
                // ถ้า ws ยังเป็น None → ยังไม่ connect
                if guard.is_none() {
                    log::error!("WebSocket not connected");
                    return;
                }
    
                // ดึงออกมา (take) ทำลาย option
                guard.take().unwrap()
            };
    
            // ตอนนี้ ws_stream คือ WebSocketStream ไม่ติด mutex แล้ว
            let (mut  write, mut read) = ws_stream.split();
    
            while let Some(msg) = read.next().await {
                if let Ok(Message::Text(txt)) = msg {
                    // !!! self ใช้ไม่ได้ใน spawn move (เพราะ self ไม่ 'static)
                    // ต้องว่าจ้าง cloned handler ก่อน
                    // แต่ตรงนี้ขอตัวอย่างแก้ concept ก่อน
                }
            }
        });
    
        Ok(())
    }
    
  
    async fn dispatch(&self, txt: &str) {
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
            Some("outboundAccountPosition") => {
                if let Ok(ev) = serde_json::from_value::<spot::Account>(event.clone()) {
                    account_handler(ev,self.account_tx.clone()).await;
                }
            }
            Some("balanceUpdate") => {
                if let Ok(ev) = serde_json::from_value::<spot::Balance>(event.clone()) {
                    balance_handler(ev,self.balance_tx.clone()).await;
                }
            }
            Some("executionReport") => {
                if let Ok(ev) = serde_json::from_value::<spot::Order>(event.clone()) {
                    order_handler(ev,self.order_tx.clone()).await;
                }
            }
            Some("eventStreamTerminated") => {
                if let Ok(ev) = serde_json::from_value::<spot::Terminated>(event.clone()) {
                    terminated_handler(ev,self.terminated_tx.clone()).await;
                }
            }
            
            _ => {}
        }
    }
}

// enum Event {
//     Trade(spot::Trade),
//     Kline(spot::Kline),
//     Order(spot::Order),
// }



// pub async fn connect_and_listen() -> anyhow::Result<()> {
//     let (ws, _) = connect_async(format!("wss://stream.binance.com:9443/ws/{}",url.to_string()) ).await?;
//     let (_, mut read) = ws.split();

//     while let Some(msg) = read.next().await {
//         if let Ok(Message::Text(json)) = msg {
//             // 🟩 step 1: รับ JSON แล้วโยนไป dispatch
//             dispatch_event(&json,).await;
//         }
//     }
//     Ok(())
// }


// use tokio::sync::mpsc;

// #[tokio::main]
// async fn main() {
//     let (tx, mut rx) = mpsc::channel::<spot::Trade>(100);

//     // spawn stream task
//     tokio::spawn(async move {
//         start_spot_stream("btcusdt", tx.clone()).await.unwrap();
//         start_kline_stream(xxxxx)
//     });

//     // spawn calculation task
//     tokio::spawn(async move {
//         while let Some(trade) = rx.recv().await {
//             process_trade(trade).await;
//         }
//     });
// }