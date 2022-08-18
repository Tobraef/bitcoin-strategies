use crate::domain::{Trade, Wallet, DollarsPerBitcoin};

use super::{Strategy, operators::{self, trade, Action}};

pub struct OnChangeBounded {
    last_val: DollarsPerBitcoin,
    exchange_ratio: f32,
    impuls_ratio: f32,
}

impl OnChangeBounded {
    /// EXCHANGE_RATIO = how much of the current wallet to transfer
    /// IMPULS_RATIO = threshold described as last / current that if crossed triggers exchange
    /// last = 10 impuls = 25 then ratio = 25 / 100 = 0.25; difference = 10 * 0.25 = 2.5;
    /// if current < 7.5 buy btc / if current > 12.5 sell
    pub fn new(exchange_ratio: f32, impuls_ratio: f32) -> Self {
        Self { 
            last_val: Default::default(), 
            exchange_ratio,
            impuls_ratio, 
        }
    }
}

impl Strategy for OnChangeBounded
{
    fn apply(&mut self, wallet: &Wallet, current_btc: DollarsPerBitcoin) -> Option<Trade> {
        if self.last_val == DollarsPerBitcoin::default() {
            self.last_val = current_btc;
            return None;
        }
        match operators::impuls(self.impuls_ratio, self.last_val, current_btc) {
            Action::DoNothing => None,
            Action::Buy => {
                self.last_val = current_btc;
                Some(trade(self.exchange_ratio, &wallet, Action::Buy))
            },
            Action::Sell => {
                self.last_val = current_btc;
                Some(trade(self.exchange_ratio, &wallet, Action::Sell))
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{domain::*, strategies::Strategy};

    use super::OnChangeBounded;

    #[test]
    fn should_trigger_with_exchange_ratio() {
        let mut sut = OnChangeBounded::new(0.1, 0.25);
        let wallet = Wallet::test_wallet();

        assert!(sut.apply(&wallet, DollarsPerBitcoin::from(10.)).is_none());

        let should_buy_btc = sut.apply(&wallet, DollarsPerBitcoin::from(6.)).unwrap();

        assert!(should_buy_btc.dollars() > Dollar::from(0.99));

        let should_sell_btc = sut.apply(&wallet, DollarsPerBitcoin::from(13.)).unwrap();
        assert!(should_sell_btc.btc() > Bitcoin::from(0.99));
    }
}