use super::{bitcoin::Bitcoin, dollar::Dollar};

pub struct Transfer {
    pub btc_change: Bitcoin,
    pub dollars_change: Dollar,
}

impl Transfer {
    pub fn new(btc_change: Bitcoin, dollars_change: Dollar) -> Self {
        Self {
            btc_change,
            dollars_change,
        }
    }
}
