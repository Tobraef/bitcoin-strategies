use std::collections::VecDeque;

#[derive(Default)]
pub struct Buffer<T, const N: usize>(VecDeque<T>);

impl<T, const N: usize> Buffer<T, N> {
    pub fn push(&mut self, t: T) {
        if self.0.len() == N {
            self.0.pop_front();
            self.0.push_back(t);
        } else {
            self.0.push_back(t);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.0.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::Buffer;

    #[test]
    fn should_contain_proper_values() {
        let mut sut = Buffer::<i32, 3>::default();

        sut.push(1);
        sut.push(5);
        sut.push(3);

        assert_eq!(sut.0, vec![1, 5, 3]);

        sut.push(2);
        sut.push(2);

        assert_eq!(sut.0, vec![3, 2, 2]);
    }
}