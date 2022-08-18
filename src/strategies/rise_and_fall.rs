use crate::domain::DollarsPerBitcoin;

use super::{buffer::Buffer, Strategy, operators::{trade, Action}};

#[derive(Default)]
pub struct RiseAndFall<const EXCHANGE: i32, const BUFFER: usize> {
    buffer: Buffer<DollarsPerBitcoin, BUFFER>,
}

impl<const EXCHANGE: i32, const N: usize> Strategy for RiseAndFall<EXCHANGE, N> {
    fn apply(&mut self, wallet: &crate::domain::Wallet, current_btc: DollarsPerBitcoin) -> Option<crate::domain::Trade> {
        self.buffer.push(current_btc);
        if self.buffer.iter().count() != N {
            return None
        }
        if is_on_rise(self.buffer.iter()) {
            Some(trade::<EXCHANGE>(&wallet, Action::Sell))
        } else if is_on_fall(self.buffer.iter()) {
            Some(trade::<EXCHANGE>(&wallet, Action::Buy))
        } else {
            None
        }
    }
}

fn is_steady_by<'a, I: Iterator<Item=&'a DollarsPerBitcoin>, F: Fn(&DollarsPerBitcoin, &DollarsPerBitcoin)->bool>(mut iter: I, f: F) -> bool {
    let mut last = iter.next().unwrap();
    while let Some(x) = iter.next() {
        if !f(&last, x) {
            return false
        } else {
            last = x;
        }
    }
    true
}

fn is_on_fall<'a, I: Iterator<Item=&'a DollarsPerBitcoin>>(iter: I) -> bool {
    is_steady_by(iter, |last,cur| last >= cur)
}

fn is_on_rise<'a, I: Iterator<Item=&'a DollarsPerBitcoin>>(iter: I) -> bool {
    is_steady_by(iter, |last, cur| last <= cur)
}

#[cfg(test)]
mod tests {
    use crate::domain::*;

    use super::*;

    #[test]
    fn should_be_on_raise_fall() {
        assert!(is_on_fall(vec![
            DollarsPerBitcoin::from(5.), 
            DollarsPerBitcoin::from(3.), 
            DollarsPerBitcoin::from(1.)].iter()));
        assert!(is_on_rise(vec![
            DollarsPerBitcoin::from(5.), 
            DollarsPerBitcoin::from(6.), 
            DollarsPerBitcoin::from(9.)].iter()));
        assert!(!is_on_fall(vec![
            DollarsPerBitcoin::from(5.), 
            DollarsPerBitcoin::from(6.), 
            DollarsPerBitcoin::from(9.)].iter()));
        assert!(!is_on_fall(vec![
            DollarsPerBitcoin::from(5.), 
            DollarsPerBitcoin::from(3.), 
            DollarsPerBitcoin::from(5.)].iter()));
        assert!(!is_on_rise(vec![
            DollarsPerBitcoin::from(5.), 
            DollarsPerBitcoin::from(3.), 
            DollarsPerBitcoin::from(1.)].iter()));
        assert!(!is_on_rise(vec![
            DollarsPerBitcoin::from(5.), 
            DollarsPerBitcoin::from(3.), 
            DollarsPerBitcoin::from(5.)].iter()));
    }

    #[test]
    fn should_perform_on_steady_rise_or_fall() {
        let mut sut = RiseAndFall::<10, 3>::default();
        let wallet = Wallet::test_wallet();

        assert!(matches!(sut.apply(&wallet, DollarsPerBitcoin::from(3.)), None));
        assert!(matches!(sut.apply(&wallet, DollarsPerBitcoin::from(5.)), None));
        assert!(matches!(sut.apply(&wallet, DollarsPerBitcoin::from(7.)), Some(Trade::Bitcoins(_))));

        assert!(sut.apply(&wallet, DollarsPerBitcoin::from(5.)).is_none());

        assert!(matches!(sut.apply(&wallet, DollarsPerBitcoin::from(3.)), Some(Trade::Dollars(_))));
    }
}

