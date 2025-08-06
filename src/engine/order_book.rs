use std::collections::BTreeMap;
use crate::engine::types::{Order, Trade, Side};

pub struct OrderBook {
    pub bids: BTreeMap<u64, Vec<Order>>, // price -> FIFO orders
    pub asks: BTreeMap<u64, Vec<Order>>, // price -> FIFO orders
}

impl OrderBook {
    pub fn new() -> Self {
        Self { bids: BTreeMap::new(), asks: BTreeMap::new() }
    }

    pub fn insert_order(&mut self, mut order: Order) -> Vec<Trade> {
        let mut trades = vec![];

        match order.side {
            Side::Buy => {
                // match against lowest ask
                while let Some((&best_price, orders)) = self.asks.iter_mut().next() {
                    if best_price > order.price || order.qty == 0 { break; }

                    let maker = orders.first_mut().unwrap();
                    let trade_qty = maker.qty.min(order.qty);

                    trades.push(Trade {
                        maker_order_id: maker.id,
                        taker_order_id: order.id,
                        price: best_price,
                        qty: trade_qty,
                    });

                    maker.qty -= trade_qty;
                    order.qty -= trade_qty;

                    if maker.qty == 0 { orders.remove(0); }
                    if orders.is_empty() { self.asks.remove(&best_price); }
                }
                if order.qty > 0 {
                    self.bids.entry(order.price).or_default().push(order);
                }
            }

            Side::Sell => {
                // match against highest bid
                while let Some((&best_price, orders)) = self.bids.iter_mut().next_back() {
                    if best_price < order.price || order.qty == 0 { break; }

                    let maker = orders.first_mut().unwrap();
                    let trade_qty = maker.qty.min(order.qty);

                    trades.push(Trade {
                        maker_order_id: maker.id,
                        taker_order_id: order.id,
                        price: best_price,
                        qty: trade_qty,
                    });

                    maker.qty -= trade_qty;
                    order.qty -= trade_qty;

                    if maker.qty == 0 { orders.remove(0); }
                    if orders.is_empty() { self.bids.remove(&best_price); }
                }
                if order.qty > 0 {
                    self.asks.entry(order.price).or_default().push(order);
                }
            }
        }

        trades
    }
}