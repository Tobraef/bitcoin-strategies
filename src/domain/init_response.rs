use serde::{Serialize, Deserialize};

use super::Wallet;

#[derive (Serialize, Deserialize)]
pub struct InitResponse {
    pub id: u32,
    pub wallet: Wallet,
}