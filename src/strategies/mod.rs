use crate::domain::{Trade, Wallet, DollarsPerBitcoin};

use self::{on_every_change::OnEveryChange, average_based::AverageBased, rise_and_fall::RiseAndFall, on_change_bounded::OnChangeBounded};

mod on_change_bounded;
mod on_every_change;
mod average_based;
mod buffer;
mod operators;
mod rise_and_fall;

pub trait Strategy: ToString {
    fn apply(&mut self, wallet: &Wallet, current_btc: DollarsPerBitcoin) -> Option<Trade>;
}

pub fn create_strategies() -> Vec<Box<dyn Strategy>> {
    let ratios = || (1..=20).map(|n| n as f32 / 10.);
    let ratios = ratios().map(|r| (r, ratios()));
    let mut result: Vec<Box<dyn Strategy>> = vec![];
    for (exchange_ratio, impuls_ratios) in ratios {
        result.push(Box::new(OnEveryChange::new(exchange_ratio)));
        result.push(Box::new(RiseAndFall::<3>::new(exchange_ratio)));
        result.push(Box::new(RiseAndFall::<6>::new(exchange_ratio)));
        result.push(Box::new(RiseAndFall::<10>::new(exchange_ratio)));
        for impuls_ratio in impuls_ratios {
            result.push(Box::new(AverageBased::<3>::new(impuls_ratio, exchange_ratio)));
            result.push(Box::new(AverageBased::<6>::new(impuls_ratio, exchange_ratio)));
            result.push(Box::new(AverageBased::<10>::new(impuls_ratio, exchange_ratio)));
            result.push(Box::new(OnChangeBounded::new(exchange_ratio, impuls_ratio)));
        }
    }
    result
}