use crate::algos::dp::{Cube, Matrix};

pub fn lcs(a: &[usize], b: &[usize]) -> usize {
    let mut dp: Matrix<usize> = Matrix::new(a.len() + 1, b.len() + 1);

    for i in 1..=a.len() {
        for j in 1..=b.len() {
            let a_prefix = dp[(i - 1, j)];
            let b_prefix = dp[(i, j - 1)];
            dp[(i, j)] = a_prefix.max(b_prefix);
            if a[i - 1] == b[j - 1] {
                let ab_prefixes = dp[(i - 1, j - 1)] + 1;
                dp[(i, j)] = dp[(i, j)].max(ab_prefixes);
            }
        }
    }
    dp[(a.len(), b.len())]
}

pub fn lcs_improved(a: &[usize], b: &[usize]) -> usize {
    let (a, b) = if a.len() < b.len() { (b, a) } else { (a, b) };

    let mut dp_previous = vec![0; b.len() + 1];
    let mut dp_current = dp_previous.clone();

    for i in 1..=a.len() {
        dp_current[0] = 0;
        for j in 1..=b.len() {
            let a_prefix = dp_previous[j];
            let b_prefix = dp_current[j - 1];
            dp_current[j] = a_prefix.max(b_prefix);
            if a[i - 1] == b[j - 1] {
                let ab_prefixes = dp_previous[j - 1] + 1;
                dp_current[j] = dp_current[j].max(ab_prefixes);
            }
        }
        std::mem::swap(&mut dp_previous, &mut dp_current);
    }
    dp_previous[b.len()]
}

pub fn lcs3(a: &[usize], b: &[usize], c: &[usize]) -> usize {
    let mut dp: Cube<usize> = Cube::new(a.len() + 1, b.len() + 1, c.len() + 1);

    for i in 1..=a.len() {
        for j in 1..=b.len() {
            for k in 1..=c.len() {
                let a_prefix = dp[(i - 1, j, k)];
                let b_prefix = dp[(i, j - 1, k)];
                let c_prefix = dp[(i, j, k - 1)];
                dp[(i, j, k)] = a_prefix.max(b_prefix).max(c_prefix);
                if a[i - 1] == b[j - 1] && a[i - 1] == c[k - 1] {
                    let abc_prefixes = dp[(i - 1, j - 1, k - 1)] + 1;
                    dp[(i, j, k)] = dp[(i, j, k)].max(abc_prefixes);
                }
            }
        }
    }
    dp[(a.len(), b.len(), c.len())]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lcs_test_internal(a: &[usize], b: &[usize], expected: usize) {
        assert_eq!(lcs(a, b), expected);
        assert_eq!(lcs_improved(a, b), expected);
    }

    #[test]
    fn lcs_test() {
        lcs_test_internal(&vec![1, 2, 5, 4, 9], &vec![18, 3, 2, 8, 0, 4, 7, 9], 3);
        lcs_test_internal(&vec![1, 1], &vec![1, 1, 1], 2);
        lcs_test_internal(&vec![13, 17, 37], &vec![37, 73, 13, 31, 23], 1);
    }

    #[test]
    fn lcs3_test() {
        assert_eq!(
            lcs3(&vec![1, 2, 3, 4], &vec![3, 2, 4, 1], &vec![3, 2, 4]),
            2
        );
        assert_eq!(lcs3(&vec![1, 2, 4], &vec![2, 4], &vec![4, 2, 2, 1]), 1);
        assert_eq!(lcs3(&vec![5, 19, 18], &vec![19, 7], &vec![1, 19, 2, 45]), 1);
    }
}
