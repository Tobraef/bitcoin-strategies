use super::{bitcoin::Bitcoin, dollar::Dollar};

pub enum Trade {
    Bitcoins(Bitcoin),
    Dollars(Dollar),
}

#[cfg(test)]
impl Trade {
    pub fn btc(self) -> Bitcoin {
        match self {
            Trade::Bitcoins(b) => b,
            Trade::Dollars(_) => panic!(),
        }
    }

    pub fn dollars(self) -> Dollar {
        match self {
            Trade::Bitcoins(_) => panic!(),
            Trade::Dollars(d) => d,
        }
    }
}