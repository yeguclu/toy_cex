mod engine;
mod bot;
// mod utils;

use engine::{ledger::Ledger, order_book::OrderBook, types::{Order, Side}};
use uuid::Uuid;
use rand::Rng;
use std::time::{Instant, Duration};
use tokio::time::sleep;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting toy CEX simulation...");

    // Shared state
    let ledger = Arc::new(Ledger::new());
    let order_book = Arc::new(Mutex::new(OrderBook::new()));


    // Initialize some fake accounts
    for account_id in 0..100000 {
        ledger.credit(account_id, "USD", 1_000_000_000); // 1M USD
        ledger.credit(account_id, "BTC", 10000);       // 100 BTC
    }

    // Spawn bots
    for _ in 0..100000 {
        let ledger = Arc::clone(&ledger);
        let order_book = Arc::clone(&order_book);
        
        tokio::spawn(async move {
            loop {
                // Create order inside a small scope
                let order = {
                    let mut rng = rand::thread_rng();
                    let account_id: u64 = rng.gen_range(0..1000);
                    let side = if rng.gen_bool(0.5) { Side::Buy } else { Side::Sell };
                    let price = rng.gen_range(99_900..100_100);

                    let usd_balance = ledger.get_balance(account_id, "USD");
                    let btc_balance = ledger.get_balance(account_id, "BTC");

                    let max_qty = match side {
                        Side::Buy => usd_balance / price,
                        Side::Sell => btc_balance,
                    };

                    if max_qty == 0 {
                        continue;
                    }

                    let qty = rng.gen_range(1..=max_qty.min(5));

                    // Skip if balance insufficient
                    if !ledger.try_place_order(account_id, side, price, qty) {
                        continue;
                    }

                    Order {
                        id: Uuid::new_v4(),
                        account_id,
                        side,
                        price,
                        qty,
                        timestamp: Instant::now(),
                    }
                };


                // Insert into order book
                {
                    let mut ob = order_book.lock().unwrap();

                    let trades = ob.insert_order(order);

                    for trade in trades {
                        // Get maker & taker orders from ledger to identify accounts
                        if let Some(maker) = ledger.open_orders.get(&trade.maker_order_id) {
                            if let Some(taker) = ledger.open_orders.get(&trade.taker_order_id) {
                                match taker.side {
                                    Side::Buy => {
                                        ledger.settle_trade(taker.account_id, maker.account_id, trade.price, trade.qty);
                                    }
                                    Side::Sell => {
                                        ledger.settle_trade(maker.account_id, taker.account_id, trade.price, trade.qty);
                                    }
                                }
                            }
                        }
                    }

                }

                // Sleep with no !Send variables in scope
                sleep(Duration::from_millis(1)).await;
            }
        });
    }

    loop {
        sleep(Duration::from_millis(100)).await;
        let ob = order_book.lock().unwrap();

        // Top 5 bids (reverse iterator for highest first)
        let top_bids: Vec<(u64, usize)> = ob.bids
            .iter()
            .rev()
            .take(5)
            .map(|(price, orders)| (*price, orders.len()))
            .collect();

        // Top 5 asks (normal iterator for lowest first)
        let top_asks: Vec<(u64, usize)> = ob.asks
            .iter()
            .take(5)
            .map(|(price, orders)| (*price, orders.len()))
            .collect();

        println!(
            "[Heartbeat] bids={} asks={} | Top 5 bids: {:?} | Top 5 asks: {:?}",
            ob.bids.len(),
            ob.asks.len(),
            top_bids,
            top_asks
        );
    }



}
