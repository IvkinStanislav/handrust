pub fn inversions_counting(mut data: Vec<usize>) -> usize {
    inversions_counting_internal(&mut data, false)
}

pub fn semi_inversions_counting(mut data: Vec<usize>) -> usize {
    inversions_counting_internal(&mut data, true)
}

fn inversions_counting_internal(data: &mut Vec<usize>, is_semi_inversions: bool) -> usize {
    if data.len() <= 1 {
        return 0;
    }

    let mut right = data.split_off(data.len() / 2);

    let left_inversions = inversions_counting_internal(data, is_semi_inversions);
    let right_inversions = inversions_counting_internal(&mut right, is_semi_inversions);
    let (new_data, merge_inversions) = merge(&data, &right, is_semi_inversions);
    *data = new_data;

    left_inversions + right_inversions + merge_inversions
}

fn merge(left: &[usize], right: &[usize], is_semi_inversions: bool) -> (Vec<usize>, usize) {
    use std::cmp::Ordering::*;

    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut left_len = left.len();
    let mut inversions_count = 0;
    let (mut left, mut right) = (left.iter(), right.iter());
    let (mut l_next, mut r_next) = (left.next(), right.next());

    loop {
        match (l_next, r_next) {
            (Some(&l), Some(&r)) => {
                if matches!(l.cmp(&r), Less) | matches!(l.cmp(&r), Equal if !is_semi_inversions) {
                    result.push(l);
                    l_next = left.next();
                    left_len -= 1;
                } else {
                    result.push(r);
                    r_next = right.next();
                    inversions_count += left_len;
                }
            }
            (Some(&e), None) | (None, Some(&e)) => {
                result.push(e);
                result.extend(left);
                result.extend(right);
                break;
            }
            (None, None) => break,
        }
    }

    (result, inversions_count)
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
        assert_eq!(inversions_counting(vec![2, 3, 9, 2]), 2);
        assert_eq!(inversions_counting(vec![1, 1, 1]), 0);
        assert_eq!(inversions_counting(vec![5, 1, 4, 2, 3, 6, 7, 9, 8]), 7);
        assert_eq!(inversions_counting(vec![5, 1, 4]), 2);
        assert_eq!(inversions_counting(vec![1, 2, 1, 2, 1, 2, 3, 1, 3, 1]), 12);
    }

    #[test]
    fn semi_inversions_counting_test() {
        assert_eq!(semi_inversions_counting(vec![2, 3, 9, 2, 9]), 4);
        assert_eq!(semi_inversions_counting(vec![1, 1, 1]), 3);
        assert_eq!(
            semi_inversions_counting(vec![1, 2, 1, 2, 1, 2, 3, 1, 3, 1]),
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
