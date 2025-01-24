pub fn binary_search(q: usize, sorted_slice: &[usize]) -> Option<usize> {
    use std::cmp::Ordering::*;

    let mut low = 0;
    let mut high = sorted_slice.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let mid_element = sorted_slice[mid];

        match q.cmp(&mid_element) {
            Equal => return Some(mid),
            Greater => low = mid + 1,
            Less => {
                if mid == 0 {
                    return None;
                }
                high = mid - 1
            }
        }
    }

    None
}

pub fn search_of_keys(keys: Vec<usize>, sorted_array: Vec<usize>) -> Vec<Option<usize>> {
    keys.into_iter()
        .map(|k| binary_search(k, &sorted_array))
        .collect()
}

pub fn counting_of_occurences(keys: Vec<usize>, sorted_array: Vec<usize>) -> Vec<usize> {
    keys.into_iter()
        .map(|k| {
            let Some(i) = binary_search_first_among_duplicates(k, &sorted_array) else {
                return 0;
            };
            number_of_consecutive_duplicates(i, &sorted_array)
        })
        .collect()
}

fn binary_search_first_among_duplicates(q: usize, sorted_slice: &[usize]) -> Option<usize> {
    use std::cmp::Ordering::*;

    let mut low = 0;
    let mut high = sorted_slice.len() - 1;
    let mut result = None;

    while low <= high {
        let mid = (low + high) / 2;
        let mid_element = sorted_slice[mid];

        match q.cmp(&mid_element) {
            Equal => {
                result = Some(mid);
                if mid == 0 {
                    break;
                }
                high = mid - 1;
            }
            Greater => low = mid + 1,
            Less => {
                if mid == 0 {
                    break;
                }
                high = mid - 1
            }
        }
    }

    result
}

fn number_of_consecutive_duplicates(mut i: usize, sorted_slice: &[usize]) -> usize {
    let target = sorted_slice[i];
    i += 1;
    let mut count = 1;
    while i < sorted_slice.len() && target == sorted_slice[i] {
        i += 1;
        count += 1;
    }
    count
}

pub fn covering_sections_min_length(k: usize, mut points: Vec<usize>) -> usize {
    points.sort_unstable();
    points.dedup();

    let mut low = 0;
    let mut high = match (points.last(), points.first()) {
        (Some(&last), Some(&first)) => last - first,
        _ => return 0,
    };

    while low <= high {
        let mid = (low + high) / 2;
        if mid == low {
            break;
        }
        let is_right = is_right_sections_length(k, mid, &points);
        if is_right {
            high = mid;
        } else {
            low = mid;
        }
    }

    if is_right_sections_length(k, low, &points) {
        low
    } else {
        high
    }
}

fn is_right_sections_length(mut k: usize, len: usize, sorted_points: &[usize]) -> bool {
    if k == 0 {
        return false;
    }
    let Some(mut current) = sorted_points.first().copied() else {
        return true;
    };

    for &point in sorted_points {
        if point > current + len {
            k -= 1;
            if k == 0 {
                return false;
            }
            current = point;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_search_test() {
        assert_eq!(binary_search(8, &vec![1, 3, 7, 8, 9, 12, 15]), Some(3));
        assert_eq!(binary_search(12, &vec![1, 3, 7, 8, 9, 12, 15]), Some(5));
        assert_eq!(binary_search(4, &vec![1, 2, 3]), None);
    }

    #[test]
    fn search_of_keys_test() {
        assert_eq!(
            search_of_keys(vec![8], vec![1, 3, 7, 8, 9, 12, 15]),
            vec![Some(3)]
        );
        assert_eq!(
            search_of_keys(vec![1, 12, 3], vec![1, 3, 7, 8, 9, 12, 15]),
            vec![Some(0), Some(5), Some(1)]
        );
        assert_eq!(
            search_of_keys(vec![1000000000, 54321, 1], vec![1, 1000000000]),
            vec![Some(1), None, Some(0)]
        );
    }

    #[test]
    fn binary_search_first_among_duplicates_test() {
        assert_eq!(
            binary_search_first_among_duplicates(3, &vec![1, 2, 2, 2, 4, 4, 9]),
            None
        );
        assert_eq!(
            binary_search_first_among_duplicates(1, &vec![1, 2, 2, 2, 4, 4, 9]),
            Some(0)
        );
        assert_eq!(
            binary_search_first_among_duplicates(2, &vec![1, 2, 2, 2, 4, 4, 9]),
            Some(1)
        );
        assert_eq!(
            binary_search_first_among_duplicates(4, &vec![1, 2, 2, 2, 4, 4, 9]),
            Some(4)
        );
    }

    #[test]
    fn counting_of_occurences_test() {
        assert_eq!(
            counting_of_occurences(vec![1, 2, 3, 4, 5], vec![1, 2, 2, 2, 4, 4, 9]),
            vec![1, 3, 0, 2, 0]
        );
    }

    #[test]
    fn covering_sections_min_length_test() {
        assert_eq!(covering_sections_min_length(3, vec![2, 3, 9, 11, 20]), 2);
        assert_eq!(covering_sections_min_length(2, vec![2, 3, 9, 11, 20]), 9);
        assert_eq!(covering_sections_min_length(2, vec![1, 2, 1, 2]), 0);
    }
}
