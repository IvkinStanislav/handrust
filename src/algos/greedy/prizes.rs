pub fn prizes_count(n: usize) -> usize {
    let d = f64::sqrt(1.0 + 8.0 * n as f64);
    let answer = (d - 1.0) / 2.0;
    answer as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn prizes_count_test() {
        assert_eq!(prizes_count(6), 3);
        assert_eq!(prizes_count(8), 3);
        assert_eq!(prizes_count(2), 1);
    }
}
