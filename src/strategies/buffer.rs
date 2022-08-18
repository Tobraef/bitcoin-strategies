use std::collections::VecDeque;

#[derive(Default)]
pub struct Buffer<F, const N: usize> {
    deq: VecDeque<F>
}

impl<F, const N: usize> Buffer<F, N>
{
    pub fn push(&mut self, val: F) {
        if self.deq.len() == N {
            self.deq.pop_front();
            self.deq.push_back(val);
        } else {
            self.deq.push_back(val);
        }
    }

    pub fn last_some<'a>(&'a self, n: usize) -> impl Iterator<Item=&'a F> {
        debug_assert!(n <= N);
        self.deq.iter().skip(self.deq.len() - n)
    }
}