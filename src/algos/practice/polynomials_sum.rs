pub fn polynomials_sum(a: Vec<usize>, b: Vec<usize>) -> Vec<usize> {
    let mut res = Vec::with_capacity(a.len().max(b.len()));
    let mut a = a.iter().rev();
    let mut b = b.iter().rev();

    loop {
        match (a.next(), b.next()) {
            (Some(item_a), Some(item_b)) => res.push(item_a + item_b),
            (Some(&item), None) => res.push(item),
            (None, Some(&item)) => res.push(item),
            (None, None) => break,
        }
    }

    res.reverse();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn polynomials_sum_test() {
        assert_eq!(
            polynomials_sum(vec![1, 2, 3, 4], vec![1, 0, 0]),
            vec![1, 3, 3, 4]
        );
        assert_eq!(
            polynomials_sum(vec![1], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1]
        );
        assert_eq!(polynomials_sum(vec![1, 1], vec![1, 1]), vec![2, 2]);
    }
}
