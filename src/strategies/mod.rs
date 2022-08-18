use crate::domain::{Dollar, Transfer, Wallet};

mod on_change_bounded;
mod on_every_change;
mod ratio;
mod buffer;

pub trait Strategy {
    fn apply(&mut self, wallet: &Wallet, current_btc: Dollar) -> Option<Transfer>;
}
