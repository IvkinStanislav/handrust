pub fn fac(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        n * fac(n - 1)
    }
}

pub fn c(n: usize, k: usize) -> usize {
    if n < k {
        0
    } else {
        fac(n) / (fac(k) * fac(n - k))
    }
}

pub fn c_repeat(n: usize, k: usize) -> usize {
    c(n + k - 1, k)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fac_test() {
        assert_eq!(fac(2), 2);
        assert_eq!(fac(1), 1);
        assert_eq!(fac(3), 6);
    }

    #[test]
    fn c_test() {
        assert_eq!(c(3, 2), 3);
        assert_eq!(c(7, 5), 21);
        assert_eq!(c(1, 1), 1);
    }

    #[test]
    fn c_repeat_test() {
        assert_eq!(c_repeat(1, 1), 1);
        assert_eq!(c_repeat(4, 3), 20);
        assert_eq!(c_repeat(2, 2), 3);
    }
}
