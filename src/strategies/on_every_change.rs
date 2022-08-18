use crate::domain::{Bitcoin, Dollar, Transfer, Wallet};

use super::Strategy;

pub struct OnEveryChange<const RATIO: i32 = 50> {
    last_val: Dollar,
}

impl<const RATIO: i32> OnEveryChange<RATIO> {
    pub fn new() -> Self {
        Self {
            last_val: (0.).into(),
        }
    }
}

impl<const RATIO: i32> Strategy for OnEveryChange<RATIO> {
    fn apply(&mut self, wallet: &Wallet, current_btc: Dollar) -> Option<Transfer> {
        debug_assert!(RATIO > 0);
        debug_assert!(RATIO < 100);
        if self.last_val == Dollar::from(0.) {
            self.last_val = current_btc;
            return None;
        }
        let f_ratio = RATIO as f32 / 100.;
        if self.last_val < current_btc {
            let to_sell = wallet.btc * f_ratio.into();
            self.last_val = current_btc;
            Some(Transfer::new(to_sell, Dollar::default()))
        } else {
            let to_buy = wallet.dollars * f_ratio.into();
            self.last_val = current_btc;
            Some(Transfer::new(Bitcoin::default(), to_buy))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{strategies::Strategy, domain::{Wallet, Bitcoin, Dollar}};

    use super::OnEveryChange;

    #[test]
    fn should_apply_if_current_is_higer_than_last() {
        let mut sut: OnEveryChange<25> = OnEveryChange::new();
        let mut wallet = Wallet::default();
        wallet.btc = Bitcoin::from(10.);
        wallet.dollars = Dollar::from(10.);

        assert!(sut.apply(&wallet, Dollar::from(5.)).is_none());
        let sold_btc = sut.apply(&wallet, Dollar::from(10.)).unwrap().btc_change;
        assert!(Into::<f32>::into(sold_btc) > 0.);
        let sold_dollars = sut.apply(&wallet, Dollar::from(5.)).unwrap().dollars_change;
        assert!(Into::<f32>::into(sold_dollars) > 0.);  
    }
}