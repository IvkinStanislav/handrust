pub fn exchange(n: usize) -> Vec<Vec<usize>> {
    todo!()
}

pub fn exchange_1_5_10_20_50(n: usize) -> Vec<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn exchange_test() {
        assert_eq!(
            exchange(10),
            vec![
                vec![10],
                vec![5, 5],
                vec![1, 1, 1, 1, 1, 5],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            ],
        );
        assert_eq!(
            exchange(1),
            vec![
                vec![1],
            ],
        );
        assert_eq!(
            exchange(5),
            vec![
                vec![5],
                vec![1, 1, 1, 1, 1],
            ],
        );
    }

    #[test]
    fn exchange_1_5_10_20_50_test() {
        assert_eq!(exchange_1_5_10_20_50(7), vec![5, 1, 1]);
        assert_eq!(exchange_1_5_10_20_50(10), vec![10]);
        assert_eq!(exchange_1_5_10_20_50(10), vec![1]);
    }
}
