use crate::domain::{Dollar, Transfer, Wallet};

use super::{Strategy, ratio::Ratio};

pub struct OnChangeBounded<
    const PANIC_RATIO: i32 = 50,
    const EXCHANGE_RATIO: i32 = 50,
    const IMPULS_RATIO: i32 = 10,
> {
    last_val: Dollar,
}

impl<const PANIC_RATIO: i32, const BUY_RATIO: i32, const IMPULS_RATIO: i32>
    OnChangeBounded<PANIC_RATIO, BUY_RATIO, IMPULS_RATIO>
{
    pub fn new() -> Self {
        Self {
            last_val: (0.).into(),
        }
    }
}

impl<const PANIC_RATIO: i32, const BUY_RATIO: i32, const IMPULS_RATIO: i32> Strategy
    for OnChangeBounded<PANIC_RATIO, BUY_RATIO, IMPULS_RATIO>
{
    fn apply(&mut self, wallet: &Wallet, current_btc: Dollar) -> Option<Transfer> {
        if self.last_val == Dollar::from(0.) {
            self.last_val = current_btc;
            return None;
        }
        let ratio = Ratio::new(self.last_val, current_btc);
        
        if ratio.rule_applies(PANIC_RATIO) {
            self.last_val = current_btc;
            Some(Transfer::new(wallet.btc, Dollar::default()))
        } else if ratio.rule_applies(IMPULS_RATIO) {
            Some(Transfer::new(btc_change, dollars_change))
        } else {
            None
        }
    }
}
