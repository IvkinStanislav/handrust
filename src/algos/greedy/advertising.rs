pub fn company(mut prices: Vec<usize>, mut clicks: Vec<usize>) -> usize {
    prices.sort_unstable_by(|a, b| b.cmp(a));
    clicks.sort_unstable_by(|a, b| b.cmp(a));

    prices.into_iter().zip(clicks).map(|(p, c)| p * c).sum()
}

pub fn billboard(n: usize, w: usize, mut applications: Vec<(usize, usize)>) -> usize {
    applications.sort_unstable_by(|(p1, _), (p2, _)| p2.cmp(p1));
    let mut w_max = n * w;
    let mut sum = 0;

    for (p, w) in applications {
        if w <= w_max {
            w_max -= w;
            sum += p * w;
        } else {
            sum += p * w_max;
            break;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn company_test() {
        assert_eq!(company(vec![23], vec![39]), 897);
        assert_eq!(company(vec![2, 3, 9], vec![7, 4, 2]), 79);
    }

    #[test]
    fn billboard_test() {
        assert_eq!(billboard(2, 3, vec![(5, 1), (2, 2), (4, 3), (1, 3)]), 21);
        assert_eq!(billboard(1, 1, vec![(1, 1)]), 1);
    }
}
