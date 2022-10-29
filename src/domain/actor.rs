use crate::strategies;

use super::Wallet;

pub struct Actor {
    pub id: u32,
    pub strategy: Box<dyn strategies::Strategy>,
    pub wallet: Wallet,
}

impl Actor {
    pub fn new(id: u32, strategy: Box<dyn strategies::Strategy>, wallet: Wallet) -> Self { Self { id, strategy, wallet } }
}