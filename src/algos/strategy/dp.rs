pub fn stones(n: usize, m: usize) -> bool {
    n % 2 != 0 || m % 2 != 0
}

pub fn stones2(n: usize, m: usize) -> bool {
    n.abs_diff(m) % 3 != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stones_test() {
        assert_eq!(stones(10, 2), false);
        assert_eq!(stones(4, 5), true);
        assert_eq!(stones(6, 8), false);
    }

    #[test]
    fn stones2_test() {
        assert_eq!(stones2(4, 4), false);
        assert_eq!(stones2(17, 72), true);
        assert_eq!(stones2(7, 5), true);
    }
}
