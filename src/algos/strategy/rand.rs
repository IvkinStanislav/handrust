pub fn lomuto(data: Vec<usize>) -> Vec<usize> {
    todo!()
}

pub fn quick_sort(data: Vec<usize>) -> Vec<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lomuto_test() {
        assert_eq!(
            lomuto(vec![4, 7, 2, 5, 3, 1, 8, 9, 6]),
            vec![1, 2, 3, 4, 7, 5, 8, 9, 6]
        );
        assert_eq!(lomuto(vec![3, 4, 7, 17]), vec![3, 4, 7, 17]);
        assert_eq!(lomuto(vec![1, 3, 2, 9, 10]), vec![1, 3, 2, 9, 10]);
    }

    #[test]
    fn quick_sort_test() {
        assert_eq!(
            quick_sort(vec![13, 17, 37, 73, 31, 19, 23]),
            vec![13, 17, 19, 23, 31, 37, 73]
        );
        assert_eq!(quick_sort(vec![18, 20, 3, 17]), vec![3, 17, 18, 20]);
        assert_eq!(quick_sort(vec![1, 11, 1]), vec![1, 1, 11]);
    }
}
