pub fn numbers_sum(a: usize, b: usize) -> usize {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_sum_test() {
        assert_eq!(numbers_sum(9, 7), 16);
    }
}
