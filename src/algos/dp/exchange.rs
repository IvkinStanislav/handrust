pub fn exchange(n: usize, mut denominations: Vec<usize>) -> usize {
    denominations.sort();
    let mut variants = vec![usize::MAX; n + 1];
    variants[0] = 0;

    for i in 1..=n {
        for &c in &denominations {
            if i < c {
                continue;
            }
            variants[i] = variants[i].min(1 + variants[i - c])
        }
    }
    variants[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exchange_1_3_4_test() {
        let denominations = vec![1, 3, 4];
        assert_eq!(exchange(18, denominations.clone()), 5);
        assert_eq!(exchange(20, denominations.clone()), 5);
        assert_eq!(exchange(34, denominations), 9);
    }
}
