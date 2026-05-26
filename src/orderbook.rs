use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Order {
    pub id: String,
    pub side: Side,
    pub price: u64,
    pub qty: u64,
}

#[derive(Clone, Copy, Debug)]
pub enum Side { Buy, Sell }

pub struct Book {
    bids: BTreeMap<u64, Vec<Order>>,
    asks: BTreeMap<u64, Vec<Order>>,
}

impl Book {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, order: Order) {
        let side = match order.side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };
        side.entry(order.price).or_default().push(order);
    }

    pub fn try_match(&mut self) -> Vec<(Order, Order)> {
        let mut trades = vec![];
        // matching price-time priority
        trades
    }
}
