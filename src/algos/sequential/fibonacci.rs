use num_bigint::BigUint;

pub fn fib(n: usize) -> BigUint {
    if n == 0 {
        return BigUint::ZERO;
    }

    let (mut a, mut b) = (BigUint::ZERO, BigUint::from(1_usize));
    for _ in 1..n {
        (b, a) = (b.clone() + a, b);
    }

    b
}

pub fn fib_last_digit(n: usize) -> usize {
    if n == 0 {
        return n;
    }

    let (mut a, mut b) = (0, 1);
    for _ in 1..n {
        (b, a) = ((b + a) % 10, b % 10);
    }

    b
}

pub fn big_fib(n: usize, m: usize) -> usize {
    let period = pizano_period_len(m);
    let res = fib(n % period) % m;
    res.try_into().unwrap()
}

fn pizano_period_len(m: usize) -> usize {
    if m <= 1 {
        return 0;
    }

    let (mut a, mut b) = (0, 1);
    let mut period_len = 1;

    loop {
        (b, a) = ((b + a) % m, b % m);
        period_len += 1;
        if b == 0 && a == 1 {
            break;
        }
    }

    period_len
}

pub fn fib_sum_last_digit(n: usize) -> usize {
    big_fib(n + 2, 10) - 1
}

pub fn fib_partial_sum_last_digit(m: usize, n: usize) -> usize {
    let mut n = fib_sum_last_digit(n);
    let m = fib_sum_last_digit(m - 1);
    if n < m {
        n += 10
    }

    n - m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_test() {
        assert_eq!(fib(3), 2_usize.into());
        assert_eq!(fib(10), 55_usize.into());
    }

    #[test]
    fn fib_last_digit_test() {
        assert_eq!(fib_last_digit(3), 2);
        assert_eq!(fib_last_digit(139), 1);
        assert_eq!(fib_last_digit(91239), 6);
    }

    #[test]
    fn big_fib_test() {
        assert_eq!(big_fib(1, 239), 1);
        assert_eq!(big_fib(115, 1000), 885);
        assert_eq!(big_fib(2816213588, 239), 151);
    }

    #[test]
    fn pizano_period_len_test() {
        assert_eq!(pizano_period_len(1), 0);
        assert_eq!(pizano_period_len(2), 3);
        assert_eq!(pizano_period_len(3), 8);
        assert_eq!(pizano_period_len(4), 6);
        assert_eq!(pizano_period_len(5), 20);
        assert_eq!(pizano_period_len(10), 60);
    }

    #[test]
    fn fib_sum_last_digit_test() {
        assert_eq!(fib_sum_last_digit(3), 4);
        assert_eq!(fib_sum_last_digit(100), 5);
    }

    #[test]
    fn fib_partial_sum_last_digit_test() {
        assert_eq!(fib_partial_sum_last_digit(3, 7), 1);
        assert_eq!(fib_partial_sum_last_digit(10, 10), 5);
    }
}
