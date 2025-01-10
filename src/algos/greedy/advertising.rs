pub fn company(prices: Vec<usize>, clicks: Vec<usize>) -> usize {
    todo!()
}

pub fn billboard(n: usize, w: usize, applications: Vec<(usize, usize)>) -> usize {
    todo!()
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
