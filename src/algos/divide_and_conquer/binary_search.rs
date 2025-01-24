pub fn binary_search(q: usize, sorted_array: Vec<usize>) -> isize {
    todo!()
}

pub fn search_of_keys(keys: Vec<usize>, sorted_array: Vec<usize>) -> Vec<isize> {
    todo!()
}

pub fn counting_of_occurences(keys: Vec<usize>, sorted_array: Vec<usize>) -> Vec<isize> {
    todo!()
}

pub fn covering_sections_min_length(k: usize, pointers: Vec<usize>) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_search_test() {
        assert_eq!(binary_search(8, vec![1, 3, 7, 8, 9, 12, 15]), 3);
        assert_eq!(binary_search(12, vec![1, 3, 7, 8, 9, 12, 15]), 5);
        assert_eq!(binary_search(4, vec![1, 2, 3]), -1);
    }

    #[test]
    fn search_of_keys_test() {
        assert_eq!(
            search_of_keys(vec![8], vec![1, 3, 7, 8, 9, 12, 15]),
            vec![3]
        );
        assert_eq!(
            search_of_keys(vec![1, 12, 3], vec![1, 3, 7, 8, 9, 12, 15]),
            vec![0, 5, 1]
        );
        assert_eq!(
            search_of_keys(vec![1000000000, 54321, 1], vec![1, 1000000000]),
            vec![1, -1, 0]
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
