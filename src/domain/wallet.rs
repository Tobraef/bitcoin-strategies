use super::{bitcoin::Bitcoin, dollar::Dollar, transfer::Transfer};

#[derive(Default)]
pub struct Wallet {
    pub btc: Bitcoin,
    pub dollars: Dollar,
}

pub fn change_balance(wallet: &mut Wallet, transfer: Transfer) {
    wallet.btc = wallet.btc + transfer.btc_change;
    wallet.dollars = wallet.dollars + transfer.dollars_change;
    if Into::<f32>::into(wallet.btc) < 0. {
        log::error!("Invalid wallet, negative btc, bringing back to 0");
        wallet.btc = (0.).into();
    }
    if Into::<f32>::into(wallet.dollars) < 0. {
        log::error!("Invalid wallet, negative dollars, bringing back to 0");
        wallet.dollars = (0.).into();
    }
}
