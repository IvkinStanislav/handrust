pub fn modification_of_quicksort(array: Vec<usize>) -> Vec<usize> {
    todo!()
}

pub fn quicksort_worst_input(n: usize) -> Vec<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modification_of_quicksort_test() {
        assert_eq!(
            modification_of_quicksort(vec![2, 3, 9, 2, 2]),
            vec![2, 2, 2, 3, 9]
        );
        assert_eq!(
            modification_of_quicksort(vec![1, 2, 3, 1]),
            vec![1, 1, 2, 3]
        );
    }

    #[test]
    fn quicksort_worst_input_test() {
        assert_eq!(quicksort_worst_input(2), vec![2, 1]);
        assert_eq!(quicksort_worst_input(4), vec![2, 4, 1, 3]);
    }
}
