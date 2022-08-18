use super::{Dollar, Bitcoin};
use derive_more::*;

#[derive(Clone, Copy, Default, PartialEq, PartialOrd, Add, Mul, Div, Sub, From)]
pub struct DollarsPerBitcoin(f32);

pub fn exchange_btc(btc: Bitcoin, price: DollarsPerBitcoin) -> Dollar {
    Dollar::from(price.0 * Into::<f32>::into(btc))
}

pub fn exchange_dollars(dollars: Dollar, price: DollarsPerBitcoin) -> Bitcoin {
    Bitcoin::from(price.0 / Into::<f32>::into(dollars))
}