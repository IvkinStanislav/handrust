pub fn select_sort(data: Vec<usize>) -> Vec<usize> {
    todo!()
}

pub fn merge(data: Vec<Vec<usize>>) -> Vec<usize> {
    todo!()
}

pub fn merge_sort(data: Vec<usize>) -> Vec<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_sort_test() {
        assert_eq!(
            select_sort(vec![13, 17, 37, 73, 31, 19, 23]),
            vec![13, 17, 19, 23, 31, 37, 73]
        );
        assert_eq!(
            select_sort(vec![12, 18, 7, 11, 5, 17]),
            vec![5, 7, 11, 12, 17, 18]
        );
        assert_eq!(select_sort(vec![1, 2, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn merge_test() {
        assert_eq!(
            merge(vec![vec![1, 2, 3], vec![1, 2], vec![3, 4, 5, 6]]),
            vec![1, 1, 2, 2, 3, 3, 5, 6, 7]
        );
        assert_eq!(
            merge(vec![vec![1, 10], vec![7, 9, 11]]),
            vec![1, 7, 9, 10, 11]
        );
        assert_eq!(
            merge(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]]),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        );
    }

    #[test]
    fn merge_sort_test() {
        assert_eq!(
            merge_sort(vec![13, 17, 37, 73, 31, 19, 23]),
            vec![13, 17, 19, 23, 31, 37, 73]
        );
        assert_eq!(merge_sort(vec![18, 20, 3, 17]), vec![3, 17, 18, 20]);
        assert_eq!(merge_sort(vec![0, 11, 0]), vec![0, 0, 11]);
    }
}
