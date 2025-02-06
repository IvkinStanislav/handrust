pub fn calculator(mut n: usize) -> Vec<usize> {
    let mut variants = vec![usize::MAX; n + 1];
    variants[0] = 0;

    for i in 1..=n {
        variants[i] = 1 + variants[i - 1];
        if i % 2 == 0 {
            variants[i] = variants[i].min(1 + variants[i / 2]);
        }
        if i % 3 == 0 {
            variants[i] = variants[i].min(1 + variants[i / 3]);
        }
    }

    let mut result = vec![];
    while n > 0 {
        result.push(n);
        if variants[n] == 1 + variants[n - 1] {
            n -= 1;
        } else if n % 2 == 0 && variants[n] == 1 + variants[n / 2] {
            n /= 2;
        } else if n % 3 == 0 && variants[n] == 1 + variants[n / 3] {
            n /= 3;
        }
    }
    result.reverse();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculator_test() {
        assert_eq!(calculator(5), vec![1, 3, 4, 5]);
        assert_eq!(calculator(10), vec![1, 3, 9, 10]);
        assert_eq!(calculator(17), vec![1, 3, 4, 8, 16, 17]);
    }
}
