pub struct FibonacciIterator {
    current: usize,
    next: usize,
    count: u8,
    size: u8,
}

impl FibonacciIterator {
    pub fn new(size: u8) -> FibonacciIterator {
        FibonacciIterator {
            current: 0,
            count: 0,
            next: 1,
            size,
        }
    }
}

impl Iterator for FibonacciIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.size {
            return None;
        }
        let fib = self.current;
        self.current = self.next;
        self.next = fib + self.next;
        self.count += 1;
        Some(fib)
    }
}

#[cfg(test)]
mod test {
    use super::FibonacciIterator;

    #[test]
    fn fibonacci() {
        let vec_fib_5 = vec![0, 1, 1, 2, 3];
        let fibonacci5 = FibonacciIterator::new(5);
        let fib_vec_5 = fibonacci5.collect::<Vec<_>>();
        assert_eq!(&vec_fib_5[..], &fib_vec_5[..]);

        let fibonacci10 = FibonacciIterator::new(10);
        let vec_fib_10 = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
        let fib_vec_10 = fibonacci10.collect::<Vec<_>>();
        assert_eq!(&vec_fib_10[..], &fib_vec_10[..]);
    }
}
