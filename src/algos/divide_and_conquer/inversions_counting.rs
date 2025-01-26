pub fn inversions_counting(data: Vec<usize>) -> usize {
    todo!()
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
