use dashmap::DashMap;
use uuid::Uuid;
use crate::engine::types::{Order, Side};

/// Ledger tracks balances and order ownership
pub struct Ledger {
    pub balances: DashMap<(u64, &'static str), u64>, // (account, asset) -> balance
    pub open_orders: DashMap<Uuid, Order>,           // order_id -> order
}

impl Ledger {
    pub fn new() -> Self {
        Self {
            balances: DashMap::new(),
            open_orders: DashMap::new(),
        }
    }

    pub fn credit(&self, account: u64, asset: &'static str, amount: u64) {
        *self.balances.entry((account, asset)).or_insert(0) += amount;
    }

    pub fn debit(&self, account: u64, asset: &'static str, amount: u64) -> bool {
        let mut entry = self.balances.entry((account, asset)).or_insert(0);
        if *entry >= amount {
            *entry -= amount;
            true
        } else {
            false
        }
    }

    pub fn try_place_order(&self, account: u64, side: Side, price: u64, qty: u64) -> bool {
        match side {
            Side::Buy => {
                let needed = price * qty;
                self.debit(account, "USD", needed)
            }
            Side::Sell => {
                self.debit(account, "BTC", qty)
            }
        }
    }

    pub fn settle_trade(&self, buyer: u64, seller: u64, price: u64, qty: u64) {
        // Buyer receives BTC
        self.credit(buyer, "BTC", qty);

        // Seller receives USD
        let usd = price * qty;
        self.credit(seller, "USD", usd);
    }

    pub fn get_balance(&self, account: u64, asset: &'static str) -> u64 {
        self.balances
            .get(&(account, asset))
            .map_or(0, |v| *v)
    }
}