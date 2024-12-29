pub fn gcd(mut a: usize, mut b: usize) -> usize {
    while a > 0 && b > 0 {
        if a >= b {
            a %= b;
        } else {
            b %= a;
        }
    }

    a.max(b)
}

pub fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

pub fn euclid_max_steps(n: usize) -> (usize, usize) {
    let (mut a, mut b) = (0, 1);
    while a <= n && b <= n {
        (a, b) = (a + b, a);
    }
    (a - b, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_test() {
        assert_eq!(gcd(18, 35), 1);
        assert_eq!(gcd(28851538, 1183019), 17657);
    }

    #[test]
    fn lcm_test() {
        assert_eq!(lcm(6, 8), 24);
        assert_eq!(lcm(761457, 614573), 467970912861);
    }

    #[test]
    fn euclid_max_steps_test() {
        assert_eq!(euclid_max_steps(3), (2, 3));
        assert_eq!(euclid_max_steps(10), (5, 8));
        assert_eq!(euclid_max_steps(10000), (4181, 6765));
    }
}
