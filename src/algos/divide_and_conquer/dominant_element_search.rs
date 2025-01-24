pub fn dominant_element_search(elements: &[usize]) -> bool {
    todo!()
}

pub fn three_dominant_element_search(elements: &[usize]) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dominant_element_search_test() {
        assert_eq!(dominant_element_search(&vec![2, 3, 9, 2, 2]), true);
        assert_eq!(dominant_element_search(&vec![1, 2, 3, 1]), false);
    }

    #[test]
    fn three_dominant_element_search_test() {
        assert_eq!(
            three_dominant_element_search(&vec![0, 9, 2, 3, 9, 0, 2, 9, 2, 3, 3]),
            true
        );
        assert_eq!(three_dominant_element_search(&vec![1, 2, 3, 1]), false);
        assert_eq!(three_dominant_element_search(&vec![0, 2, 1]), true);
        assert_eq!(
            three_dominant_element_search(&vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2]),
            false
        );
    }
}
