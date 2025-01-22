pub fn max_salary(numbers: Vec<usize>) -> usize {
    todo!()
}

pub fn tennis_tournament(k: usize, abilities: Vec<usize>) -> usize {
    todo!()
}

pub fn chess_kings(r: usize, c: usize) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn max_salary_test() {
        assert_eq!(max_salary(vec![21, 2]), 221);
        assert_eq!(max_salary(vec![9, 4, 6, 1, 9]), 99641);
        assert_eq!(max_salary(vec![23, 39, 92]), 923923);
    }

    #[test]
    #[rustfmt::skip]
    fn tennis_tournament_test() {
        assert_eq!(tennis_tournament(2, vec![1, 2, 4, 3]), 1);
        assert_eq!(tennis_tournament(3, vec![1, 2, 3, 4, 5]), 2);
        assert_eq!(tennis_tournament(2, vec![1, 2, 3, 4]), 1);
    }

    #[test]
    #[rustfmt::skip]
    fn chess_kings_test() {
        assert_eq!(chess_kings(5, 1), 3);
        assert_eq!(chess_kings(3, 3), 8);
        assert_eq!(chess_kings(4, 4), 12);
    }
}
