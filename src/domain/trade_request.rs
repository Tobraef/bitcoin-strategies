use serde::{Serialize, Deserialize};

use super::Trade;

#[derive(Serialize, Deserialize)]
pub struct TradeRequest {
    pub actor_id: u32,
    pub trade: Trade,
}

impl TradeRequest {
    pub fn new(actor_id: u32, trade: Trade) -> Self { Self { actor_id, trade } }
}