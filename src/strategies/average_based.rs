use crate::domain::{Wallet, DollarsPerBitcoin, Trade};

use super::{buffer::Buffer, Strategy, operators::{impuls, Action, trade}};

#[derive(Default)]
pub struct AverageBased<const BUFFER_SIZE: usize, const IMPULS_RATIO: i32, const EXCHANGE_RATIO: i32> {
    buffer: Buffer<DollarsPerBitcoin, BUFFER_SIZE>,
}

impl<const N: usize, const IMPULS: i32, const EXCHANGE: i32> Strategy for AverageBased<N, IMPULS, EXCHANGE> {
    fn apply(&mut self, wallet: &Wallet, current_btc: DollarsPerBitcoin) -> Option<Trade> {
        self.buffer.push(current_btc);
        let avg = average(self.buffer.iter());
        match impuls::<IMPULS>(avg, current_btc) {
            Action::DoNothing => None,
            e => Some(trade::<EXCHANGE>(&wallet, e)),
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