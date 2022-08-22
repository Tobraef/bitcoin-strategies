use super::{Dollar, Bitcoin};
use derive_more::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd, Add, Mul, Div, Sub, From)]
pub struct DollarsPerBitcoin(f32);

pub fn exchange_btc(btc: Bitcoin, price: DollarsPerBitcoin) -> Dollar {
    Dollar::from(price.0 * Into::<f32>::into(btc))
}

pub fn exchange_dollars(dollars: Dollar, price: DollarsPerBitcoin) -> Bitcoin {
    Bitcoin::from(Into::<f32>::into(dollars) / price.0)
}