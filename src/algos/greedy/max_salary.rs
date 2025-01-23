use std::usize;

pub fn max_salary(mut numbers: Vec<usize>) -> usize {
    numbers.sort_unstable_by(|&n1, &n2| max_salary_comparator(n1, n2));

    numbers.into_iter().rev().fold(0, |salary, n| {
        let number_of_digits = n.checked_ilog10().unwrap_or_default() + 1;
        salary * 10_usize.pow(number_of_digits) + n
    })
}

fn max_salary_comparator(n1: usize, n2: usize) -> std::cmp::Ordering {
    let (first_digit1, remainder1) = first_digit_and_remainder(n1);
    let (first_digit2, remainder2) = first_digit_and_remainder(n2);

    use std::cmp::Ordering::*;
    match first_digit1.cmp(&first_digit2) {
        Equal => match (remainder1, remainder2) {
            (Some(remainder1), Some(remainder2)) => max_salary_comparator(remainder1, remainder2),
            (Some(_), None) => Less,
            (None, Some(_)) => Greater,
            (None, None) => Equal,
        },
        less_or_greater => less_or_greater,
    }
}

fn first_digit_and_remainder(n: usize) -> (usize, Option<usize>) {
    if n < 10 {
        (n, None)
    } else {
        let power = 10_usize.pow(n.ilog10());
        (n / power, Some(n % power))
    }
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
    fn first_digit_and_remainder_test() {
        assert_eq!(first_digit_and_remainder(0), (0, None));
        assert_eq!(first_digit_and_remainder(7), (7, Some(0)));
        assert_eq!(first_digit_and_remainder(28), (2, Some(8)));
        assert_eq!(first_digit_and_remainder(564), (5, Some(64)));
    }

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
