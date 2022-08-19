use crate::domain::{Wallet, DollarsPerBitcoin, Trade};

use super::{buffer::Buffer, Strategy, operators::{impuls, Action, trade}};

#[derive(Default)]
pub struct AverageBased<const BUFFER_SIZE: usize> {
    buffer: Buffer<DollarsPerBitcoin, BUFFER_SIZE>,
    impuls_ratio: f32,
    exchange_ratio: f32,
}

impl<const N: usize> AverageBased<N> {
    pub fn new(impuls_ratio: f32, exchange_ratio: f32) -> Self {
        Self {
            impuls_ratio,
            exchange_ratio,
            buffer: Buffer::default(),
        }
    }
}

impl<const N: usize> ToString for AverageBased<N> {
    fn to_string(&self) -> String {
        format!("Average based with {N} buffer size, {} impuls ratio, {} exchange_ratio", self.impuls_ratio, self.exchange_ratio)
    }
}

impl<const N: usize> Strategy for AverageBased<N> {
    fn apply(&mut self, wallet: &Wallet, current_btc: DollarsPerBitcoin) -> Option<Trade> {
        self.buffer.push(current_btc);
        let avg = average(self.buffer.iter());
        match impuls(self.impuls_ratio, avg, current_btc) {
            Action::DoNothing => None,
            e => Some(trade(self.exchange_ratio, &wallet, e)),
        }
    }
}

fn average<'a, I: Iterator<Item=&'a DollarsPerBitcoin>>(mut iter: I) -> DollarsPerBitcoin {
    let mut sum = DollarsPerBitcoin::from(0.);
    let mut count = 0;
    while let Some(&x) = iter.next() {
        count += 1;
        sum = sum + x;
    }
    sum / count as f32
}