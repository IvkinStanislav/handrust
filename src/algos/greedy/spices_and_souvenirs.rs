pub fn spices(w: usize, counts: Vec<(usize, usize)>) -> f64 {
    todo!()
}

pub fn souvenirs(s: usize, counts: Vec<usize>) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn spices_test() {
        assert_eq!(spices(50, vec![(60, 20), (100, 50), (120, 30)]), 180.000);
        assert_eq!(spices(10, vec![(500, 30)]), 166.667);
        assert_eq!(spices(1000, vec![(500, 30)]), 500.000);
    }

    #[test]
    fn souvenirs_test() {
        assert_eq!(souvenirs(50, vec![20, 50, 30]), 20);
        assert_eq!(souvenirs(1, vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1]), 6);
        assert_eq!(souvenirs(10, vec![500]), 0);
    }
}
