use crate::{domain::{Trade, Wallet, DollarsPerBitcoin}, strategies::operators::{trade, Action}};

use super::Strategy;

pub struct OnEveryChange {
    last_val: DollarsPerBitcoin,
    exchange_ratio: f32,
}

impl OnEveryChange {
    pub fn new(exchange_ratio: f32) -> Self {
        debug_assert!(exchange_ratio > 0.);
        debug_assert!(exchange_ratio <= 100.); 
        Self { 
            last_val: Default::default(), 
            exchange_ratio,
        } 
    } 
}

impl ToString for OnEveryChange {
    fn to_string(&self) -> String {
        format!("On every change with {} exchange_ratio", self.exchange_ratio)
    }
}

impl Strategy for OnEveryChange {
    fn apply(&mut self, wallet: &Wallet, current_btc: DollarsPerBitcoin) -> Option<Trade> {
        if self.last_val == DollarsPerBitcoin::default() {
            self.last_val = current_btc;
            return None;
        }
        if self.last_val < current_btc {
            self.last_val = current_btc;
            Some(trade(self.exchange_ratio, &wallet, Action::Sell))
        } else {
            self.last_val = current_btc;
            Some(trade(self.exchange_ratio, &wallet, Action::Buy))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{strategies::Strategy, domain::{Wallet, DollarsPerBitcoin, Dollar, Trade, Bitcoin}};

    use super::OnEveryChange;

    #[test]
    fn should_apply_if_current_is_higer_than_last() {
        let mut sut = OnEveryChange::new(0.25);
        let wallet = Wallet::test_wallet();

        assert!(sut.apply(&wallet, DollarsPerBitcoin::from(5.)).is_none());
        let sold_btc = sut.apply(&wallet, DollarsPerBitcoin::from(10.)).unwrap().btc();
        assert!(Into::<f32>::into(sold_btc) > 0.);
        let sold_dollars = sut.apply(&wallet, DollarsPerBitcoin::from(5.)).unwrap().dollars();
        assert!(Into::<f32>::into(sold_dollars) > 0.);  
    }

    #[test]
    fn should_keep_exchanging() {
        let mut sut = OnEveryChange::new(0.5);
        let mut wallet = Wallet::test_wallet();

        sut.apply(&wallet, DollarsPerBitcoin::from(10.));
        let to_trade = sut.apply(&wallet, DollarsPerBitcoin::from(11.)).unwrap();
        assert_eq!(to_trade.btc(), Bitcoin::from(5.));
        wallet.btc = wallet.btc - Bitcoin::from(5.);
        
        let to_trade = sut.apply(&wallet, DollarsPerBitcoin::from(12.)).unwrap();
        assert_eq!(to_trade.btc(), Bitcoin::from(2.5));
        wallet.btc = wallet.btc - Bitcoin::from(2.5);
        
        let to_trade = sut.apply(&wallet, DollarsPerBitcoin::from(13.)).unwrap();
        assert_eq!(to_trade.btc(), Bitcoin::from(1.25));
        wallet.btc = wallet.btc - Bitcoin::from(1.25);

        assert_eq!(wallet.btc, Bitcoin::from(1.25));
    }
}