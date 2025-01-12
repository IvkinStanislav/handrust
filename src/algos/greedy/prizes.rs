pub fn prizes_count(n: usize) -> usize {
    todo!()
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
