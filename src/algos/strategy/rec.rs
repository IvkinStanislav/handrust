pub fn towers(n: usize) -> Vec<(usize, usize)> {
    todo!()
}

pub fn towers2(n: usize) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn towers_test() {
        assert_eq!(
            towers(3),
            vec![(1, 3), (1, 2), (3, 2), (1, 3), (2, 1), (2, 3), (1, 3)]
        );
        assert_eq!(
            towers(4),
            vec![
                (1, 2),
                (1, 3),
                (2, 3),
                (1, 2),
                (3, 1),
                (3, 2),
                (1, 2),
                (1, 3),
                (2, 3),
                (2, 1),
                (3, 1),
                (2, 3),
                (1, 2),
                (1, 3),
                (2, 3)
            ]
        );
        assert_eq!(
            towers(5),
            vec![
                (1, 3),
                (1, 2),
                (3, 2),
                (1, 3),
                (2, 1),
                (2, 3),
                (1, 3),
                (1, 2),
                (3, 2),
                (3, 1),
                (2, 1),
                (3, 2),
                (1, 3),
                (1, 2),
                (3, 2),
                (1, 3),
                (2, 1),
                (2, 3),
                (1, 3),
                (2, 1),
                (3, 2),
                (3, 1),
                (2, 1),
                (2, 3),
                (1, 3),
                (1, 2),
                (3, 2),
                (1, 3),
                (2, 1),
                (2, 3),
                (1, 3)
            ]
        );
    }

    #[test]
    fn towers2_test() {
        assert_eq!(towers2(3), 5);
        assert_eq!(towers2(4), 9);
        assert_eq!(towers2(5), 13);
    }
}
