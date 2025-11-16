use crate::stream::binance::spot;
use tokio::sync::mpsc::Sender;
use log;

pub async fn account_handler(event: spot::Account, tx: Sender<spot::Account>) {
    
    if 1.0 > 100000.0 {
        log::info!("{:?} price too high, skipping", &event);
    } else {
        // ส่งข้อมูลไปยัง calculation module
        if let Err(e) = tx.send(event).await {
            log::error!("Failed to send trade event: {}", e);
        }
    }
}

pub async fn trade_handler(event: spot::Trade, tx: Sender<spot::Trade>) {
    
    if 1.0 > 100000.0 {
        log::info!("{} price too high, skipping", &event.symbol);
    } else {
        // ส่งข้อมูลไปยัง calculation module
        if let Err(e) = tx.send(event).await {
            log::error!("Failed to send trade event: {}", e);
        }
    }
}

pub async fn kline_handler(event: spot::Kline, tx: Sender<spot::Kline>) {
    let symbol = event.symbol.clone();

    if 1.0 > 100000.0 {
        log::info!("{} price too high, skipping", symbol);
    } else {
        // ส่งข้อมูลไปยัง calculation module
        if let Err(e) = tx.send(event).await {
            log::error!("Failed to send trade event: {}", e);
        }
    }
}

pub async fn terminated_handler(event: spot::Terminated, tx: Sender<spot::Terminated>) {

    if 1.0 > 100000.0 {
        log::info!("price too high, skipping");
    } else {
        // ส่งข้อมูลไปยัง calculation module
        if let Err(e) = tx.send(event).await {
            log::error!("Failed to send trade event: {}", e);
        }
    }
}

pub async fn balance_handler(event: spot::Balance, tx: Sender<spot::Balance>) {
    if 1.0 > 100000.0 {
        log::info!("price too high, skipping");
    } else {
        // ส่งข้อมูลไปยัง calculation module
        if let Err(e) = tx.send(event).await {
            log::error!("Failed to send trade event: {}", e);
        }
    }
}

pub async fn order_handler(event: spot::Order, tx: Sender<spot::Order>) {

    if 1.0 > 100000.0 {
        log::info!("price too high, skipping");
    } else {
        // ส่งข้อมูลไปยัง calculation module
        if let Err(e) = tx.send(event).await {
            log::error!("Failed to send trade event: {}", e);
        }
    }
}
