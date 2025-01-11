pub fn collection_of_signatures(coordinates: Vec<(usize, usize)>) -> Vec<usize> {
    todo!()
}

pub fn covering(l: usize, coordinates: Vec<usize>) -> usize {
    todo!()
}

pub fn min_segments_sum(k: usize, coordinates: Vec<usize>) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn collection_of_signatures_test() {
        assert_eq!(collection_of_signatures(vec![(1, 3), (2, 5), (3, 6)]), vec![3]);
        assert_eq!(collection_of_signatures(vec![(4, 7), (1, 3), (2, 5), (3, 6)]), vec![3, 7]);
    }

    #[test]
    fn covering_test() {
        assert_eq!(covering(4, vec![1, 1, 3, 4, 5]), 1);
        assert_eq!(covering(1, vec![1, 2, 3, 4, 5]), 3);
    }

    #[test]
    fn min_segments_sum_test() {
        assert_eq!(min_segments_sum(2, vec![1, 1, 3, 4, 5]), 2);
        assert_eq!(min_segments_sum(1000000000, vec![1, 1, 3, 4, 5]), 0);
        assert_eq!(
            min_segments_sum(3, vec![1, 4, 2, 10, 20, 11, 12, 14, 19, 15]),
            9
        );
    }
}
