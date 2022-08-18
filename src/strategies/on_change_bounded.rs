use crate::domain::{Trade, Wallet, DollarsPerBitcoin};

use super::{Strategy, operators::{self, trade, Action}};

/// EXCHANGE_RATIO = how much of the current wallet to transfer
/// IMPULS_RATIO = threshold described as last / current that if crossed triggers exchange
/// last = 10 impuls = 25 then ratio = 25 / 100 = 0.25; difference = 10 * 0.25 = 2.5;
/// if current < 7.5 buy btc / if current > 12.5 sell
#[derive(Default)]
pub struct OnChangeBounded<
    const EXCHANGE_RATIO: i32 = 50,
    const IMPULS_RATIO: i32 = 10,
> {
    last_val: DollarsPerBitcoin,
}

impl<const EXCHANGE_RATIO: i32, const IMPULS_RATIO: i32> Strategy
    for OnChangeBounded<EXCHANGE_RATIO, IMPULS_RATIO>
{
    fn apply(&mut self, wallet: &Wallet, current_btc: DollarsPerBitcoin) -> Option<Trade> {
        if self.last_val == DollarsPerBitcoin::default() {
            self.last_val = current_btc;
            return None;
        }
        match operators::impuls::<IMPULS_RATIO>(self.last_val, current_btc) {
            Action::DoNothing => None,
            Action::Buy => {
                self.last_val = current_btc;
                Some(trade::<EXCHANGE_RATIO>(&wallet, Action::Buy))
            },
            Action::Sell => {
                self.last_val = current_btc;
                Some(trade::<EXCHANGE_RATIO>(&wallet, Action::Sell))
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
        let mut sut: OnChangeBounded<10, 25> = OnChangeBounded::default();
        let wallet = Wallet::test_wallet();

        assert!(sut.apply(&wallet, DollarsPerBitcoin::from(10.)).is_none());

        let should_buy_btc = sut.apply(&wallet, DollarsPerBitcoin::from(6.)).unwrap();

        assert!(should_buy_btc.dollars() > Dollar::from(0.99));

        let should_sell_btc = sut.apply(&wallet, DollarsPerBitcoin::from(13.)).unwrap();
        assert!(should_sell_btc.btc() > Bitcoin::from(0.99));
    }
}