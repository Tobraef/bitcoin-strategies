use super::{bitcoin::Bitcoin, dollar::Dollar, trade::Trade, dollars_per_bitcoin::{DollarsPerBitcoin, exchange_btc, exchange_dollars}};

#[derive(Default)]
pub struct Wallet {
    pub btc: Bitcoin,
    pub dollars: Dollar,
}

fn validate(wallet: &mut Wallet) {
    if *wallet.btc.as_ref() < 0. {
        log::error!("Invalid wallet, negative btc, bringing back to 0");
        wallet.btc = Bitcoin::from(0.);
    }
    if *wallet.dollars.as_ref() < 0. {
        log::error!("Invalid wallet, negative dollars, bringing back to 0");
        wallet.dollars = Dollar::from(0.);
    }
}

/// Changes the wallet according to made transfer.
/// If values would go negative, they are brought to 0 and log an error
pub fn change_balance(wallet: &mut Wallet, transfer: Trade, price: DollarsPerBitcoin) {
    match transfer {
        Trade::Bitcoins(btc) => {
            let dollars = exchange_btc(btc, price);
            wallet.btc = wallet.btc - btc;
            wallet.dollars = wallet.dollars + dollars;
        },
        Trade::Dollars(dl) => {
            let btc = exchange_dollars(dl, price);
            wallet.dollars = wallet.dollars - dl;
            wallet.btc = wallet.btc + btc;
        },
    }
    validate(wallet)
}

impl Wallet {
    #[cfg(test)]
    pub fn test_wallet() -> Wallet {
        Wallet { btc: Bitcoin::from(10.), dollars: Dollar::from(10.) }
    }
}
