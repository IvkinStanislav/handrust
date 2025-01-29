use std::iter::Peekable;

pub fn inversions_counting(data: Vec<usize>) -> usize {
    todo!()
}

struct MergeIterator<I: Iterator> {
    iter1: Peekable<I>,
    iter2: Peekable<I>,
}

impl<I: Iterator> MergeIterator<I> {
    fn new<II: IntoIterator<IntoIter = I>>(source1: II, source2: II) -> Self {
        Self {
            iter1: source1.into_iter().peekable(),
            iter2: source2.into_iter().peekable(),
        }
    }
}

impl<T: Ord, I: Iterator<Item = T>> Iterator for MergeIterator<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        use std::cmp::Ordering::*;

        match (self.iter1.peek(), self.iter2.peek()) {
            (Some(e1), Some(e2)) => match e1.cmp(&e2) {
                Less | Equal => self.iter1.next(),
                Greater => self.iter2.next(),
            },
            (Some(_), None) => self.iter1.next(),
            (None, Some(_)) => self.iter2.next(),
            (None, None) => None,
        }
    }
}

pub fn semi_inversion_counting(data: Vec<usize>) -> usize {
    todo!()
}

/// ascending
pub fn transposition_counting_1(data: Vec<usize>) -> usize {
    todo!()
}

/// x,x+1,…,n,1,2,…,x−1.
pub fn transposition_counting_2(data: Vec<usize>) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inversions_counting_test() {
        assert_eq!(inversions_counting(vec![2, 3, 9, 2, 9]), 2);
        assert_eq!(inversions_counting(vec![1, 1, 1]), 0);
        assert_eq!(inversions_counting(vec![1, 2, 1, 2, 1, 2, 3, 1, 3, 1]), 12);
    }

    #[test]
    fn semi_inversion_counting_test() {
        assert_eq!(semi_inversion_counting(vec![2, 3, 9, 2, 9]), 4);
        assert_eq!(semi_inversion_counting(vec![1, 1, 1]), 3);
        assert_eq!(
            semi_inversion_counting(vec![1, 2, 1, 2, 1, 2, 3, 1, 3, 1]),
            26
        );
    }

    #[test]
    fn transposition_counting_test() {
        assert_eq!(transposition_counting_1(vec![3, 5, 4, 2, 1]), 8);
    }

    #[test]
    fn _test() {
        assert_eq!(transposition_counting_2(vec![3, 5, 4, 2, 1]), 2);
    }
}
