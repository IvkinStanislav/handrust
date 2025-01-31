pub fn inversions_counting(data: &mut Vec<usize>) -> usize {
    if data.len() <= 1 {
        return 0;
    }

    let (left, right) = data.split_at(data.len() / 2);
    let (mut left, mut right) = (left.to_vec(), right.to_vec());

    let left_inversions = inversions_counting(&mut left);
    let right_inversions = inversions_counting(&mut right);
    let (new_data, merge_inversions) = merge(&left, &right);
    *data = new_data;

    left_inversions + right_inversions + merge_inversions
}

fn merge(left: &[usize], right: &[usize]) -> (Vec<usize>, usize) {
    use std::cmp::Ordering::*;

    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut left_len = left.len();
    let mut inversions_count = 0;
    let (mut left, mut right) = (left.iter(), right.iter());
    let (mut l_next, mut r_next) = (left.next(), right.next());

    loop {
        match (l_next, r_next) {
            (Some(&l), Some(&r)) => match l.cmp(&r) {
                Less | Equal => {
                    result.push(l);
                    l_next = left.next();
                    left_len -= 1;
                }
                Greater => {
                    inversions_count += left_len;
                    result.push(r);
                    r_next = right.next();
                }
            },
            (Some(&l), None) => {
                result.push(l);
                l_next = left.next();
            }
            (None, Some(&r)) => {
                result.push(r);
                r_next = right.next();
            }
            (None, None) => break,
        }
    }

    (result, inversions_count)
}

pub fn semi_inversion_counting(data: Vec<usize>) -> usize {
    todo!()
}

/// ascending
pub fn transposition_counting_1(data: Vec<usize>) -> usize {
    todo!()
}

/// x,x+1,…,n,1,2,…,x−1.
pub fn transposition_counting_2(data: Vec<usize>) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inversions_counting_test() {
        assert_eq!(inversions_counting(&mut vec![2, 3, 9, 2]), 2);
        assert_eq!(inversions_counting(&mut vec![1, 1, 1]), 0);
        assert_eq!(inversions_counting(&mut vec![5, 1, 4, 2, 3, 6, 7, 9, 8]), 7);
        assert_eq!(inversions_counting(&mut vec![5, 1, 4]), 2);
        assert_eq!(
            inversions_counting(&mut vec![1, 2, 1, 2, 1, 2, 3, 1, 3, 1]),
            12
        );
    }

    #[test]
    fn semi_inversion_counting_test() {
        assert_eq!(semi_inversion_counting(vec![2, 3, 9, 2, 9]), 4);
        assert_eq!(semi_inversion_counting(vec![1, 1, 1]), 3);
        assert_eq!(
            semi_inversion_counting(vec![1, 2, 1, 2, 1, 2, 3, 1, 3, 1]),
            26
        );
    }

    #[test]
    fn transposition_counting_test() {
        assert_eq!(transposition_counting_1(vec![3, 5, 4, 2, 1]), 8);
    }

    #[test]
    fn _test() {
        assert_eq!(transposition_counting_2(vec![3, 5, 4, 2, 1]), 2);
    }
}
