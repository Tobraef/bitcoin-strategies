use crate::{domain::{Trade, Wallet, DollarsPerBitcoin}, strategies::operators::{trade, Action}};

use super::Strategy;

#[derive(Default)]
pub struct OnEveryChange<const RATIO: i32 = 50> {
    last_val: DollarsPerBitcoin,
}

impl<const RATIO: i32> Strategy for OnEveryChange<RATIO> {
    fn apply(&mut self, wallet: &Wallet, current_btc: DollarsPerBitcoin) -> Option<Trade> {
        debug_assert!(RATIO > 0);
        debug_assert!(RATIO < 100);
        if self.last_val == DollarsPerBitcoin::default() {
            self.last_val = current_btc;
            return None;
        }
        if self.last_val < current_btc {
            self.last_val = current_btc;
            Some(trade::<RATIO>(&wallet, Action::Sell))
        } else {
            self.last_val = current_btc;
            Some(trade::<RATIO>(&wallet, Action::Buy))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{strategies::Strategy, domain::{Wallet, DollarsPerBitcoin}};

    use super::OnEveryChange;

    #[test]
    fn should_apply_if_current_is_higer_than_last() {
        let mut sut: OnEveryChange<25> = OnEveryChange::default();
        let wallet = Wallet::test_wallet();

        assert!(sut.apply(&wallet, DollarsPerBitcoin::from(5.)).is_none());
        let sold_btc = sut.apply(&wallet, DollarsPerBitcoin::from(10.)).unwrap().btc();
        assert!(Into::<f32>::into(sold_btc) > 0.);
        let sold_dollars = sut.apply(&wallet, DollarsPerBitcoin::from(5.)).unwrap().dollars();
        assert!(Into::<f32>::into(sold_dollars) > 0.);  
    }
}