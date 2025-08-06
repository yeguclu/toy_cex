use std::time::Instant;
use uuid::Uuid;

#[derive(Clone, Copy, Debug)]
pub enum Side { Buy, Sell }

#[derive(Debug)]
pub struct Order {
    pub id: Uuid,
    pub account_id: u64,
    pub side: Side,
    pub price: u64,
    pub qty: u64,
    pub timestamp: Instant,
}

#[derive(Debug)]
pub struct Trade {
    pub maker_order_id: Uuid,
    pub taker_order_id: Uuid,
    pub price: u64,
    pub qty: u64,
}