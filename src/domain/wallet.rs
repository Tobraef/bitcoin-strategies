use serde::{Deserialize, Serialize};

use super::{bitcoin::Bitcoin, dollar::Dollar, trade::Trade, dollars_per_bitcoin::{DollarsPerBitcoin, exchange_btc, exchange_dollars}};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Wallet {
    pub id: u32,
    pub btc: Bitcoin,
    pub dollars: Dollar,
}

impl Wallet {
    pub fn new(id: u32, btc: Bitcoin, dollars: Dollar) -> Self { Self { id, btc, dollars } }

    #[cfg(test)]
    pub fn test_wallet() -> Wallet {
        Wallet { btc: Bitcoin::from(10.), dollars: Dollar::from(10.), id: 0 }
    }
}
