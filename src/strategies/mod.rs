use crate::domain::{Trade, Wallet, DollarsPerBitcoin};

mod on_change_bounded;
mod on_every_change;
mod average_based;
mod buffer;
mod operators;
mod rise_and_fall;

pub trait Strategy {
    fn apply(&mut self, wallet: &Wallet, current_btc: DollarsPerBitcoin) -> Option<Trade>;
}
