use serde::{Serialize, Deserialize};

use super::Wallet;

#[derive(Serialize, Deserialize)]
pub struct TradeResponse {
    pub actor_id: u32,
    pub wallet: Wallet,
}

impl TradeResponse {
    pub fn new(actor_id: u32, wallet: Wallet) -> Self { Self { actor_id, wallet } }
}