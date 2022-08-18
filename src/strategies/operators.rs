use crate::domain::{Wallet, Trade, Dollar, Bitcoin, DollarsPerBitcoin};

#[derive(PartialEq, Eq, Debug)]
pub enum Action {
    DoNothing,
    Buy,
    Sell,
}

pub fn impuls(impuls_ratio: f32, last_val: DollarsPerBitcoin, current_val: DollarsPerBitcoin) -> Action {
    let difference = last_val * impuls_ratio;

    if last_val - difference > current_val {
        Action::Buy
    } else if last_val + difference < current_val {
        Action::Sell
    } else {
        Action::DoNothing
    }
}

pub fn trade(ratio: f32, wallet: &Wallet, impuls_decision: Action) -> Trade {
    match impuls_decision {
        Action::DoNothing => unreachable!("Should be handled already"),
        Action::Buy => {
            let to_transfer: Dollar = wallet.dollars * ratio;
            Trade::Dollars(to_transfer)
        },
        Action::Sell => {
            let to_transfer: Bitcoin = wallet.btc * ratio;
            Trade::Bitcoins(to_transfer)
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn impuls_should_handle_values_properly() {
        assert_eq!(impuls(0.25, DollarsPerBitcoin::from(10.), DollarsPerBitcoin::from(7.)), Action::Buy);
        assert_eq!(impuls(0.25, DollarsPerBitcoin::from(10.), DollarsPerBitcoin::from(13.)), Action::Sell);
        assert_eq!(impuls(0.25, DollarsPerBitcoin::from(10.), DollarsPerBitcoin::from(8.)), Action::DoNothing);
        assert_eq!(impuls(0.25, DollarsPerBitcoin::from(10.), DollarsPerBitcoin::from(12.)), Action::DoNothing);
        assert_eq!(impuls(0.15, DollarsPerBitcoin::from(10.), DollarsPerBitcoin::from(8.)), Action::Buy);
        assert_eq!(impuls(0.15, DollarsPerBitcoin::from(10.), DollarsPerBitcoin::from(12.)), Action::Sell);
    }

    #[test]
    fn trade_should_buy_btc_on_buy_and_sell_on_sell() {
        let wallet = Wallet::test_wallet();
        match trade(0.5, &wallet, Action::Buy) {
            Trade::Bitcoins(_) => panic!(),
            Trade::Dollars(d) => 
                assert!(0f32 < d.into()),
        }
        match trade(0.5, &wallet, Action::Sell) {
            Trade::Bitcoins(b) => assert!(0f32 < b.into()),
            Trade::Dollars(_) => panic!(),
        }
    }
}