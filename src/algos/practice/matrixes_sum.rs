pub fn matrixes_sum(a: Vec<isize>, b: Vec<isize>) -> Vec<isize> {
    a.iter()
        .zip(b.iter())
        .map(|(item_a, item_b)| item_a + item_b)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn matrixes_sum_test() {
        assert_eq!(
            matrixes_sum(
                vec![
                    1, 2, 3,
                    4, 5, 6,
                ],
                vec![
                    -1, -2, -3,
                    -4, -5, -6,
                ]
            ),
            vec![
                0, 0, 0,
                0, 0, 0,
            ]
        );
        assert_eq!(
            matrixes_sum(
                vec![
                    1, 2, 3,
                    4, 5, 6,
                ],
                vec![
                    1, 2, 3,
                    4, 5, 6,
                ]
            ),
            vec![
                2, 4, 6,
                8, 10, 12,
            ]
        );
        assert_eq!(
            matrixes_sum(
                vec![
                    1, 2, 3,
                ],
                vec![
                    4, 5, 6,
                ]
            ),
            vec![
                5, 7, 9,
            ]
        );
    }
}
