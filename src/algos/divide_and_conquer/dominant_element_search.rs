use std::collections::HashMap;

pub fn dominant_element_search(elements: &[usize]) -> bool {
    let mut dominant = 0;
    let mut count = 0;

    for &element in elements {
        if count == 0 {
            dominant = element;
            count += 1;
        } else if dominant == element {
            count += 1;
        } else {
            count -= 1;
        }
    }

    count > 0
}

pub fn k_dominants_element_search(k: usize, elements: &[usize]) -> bool {
    let mut dominants: HashMap<usize, usize> = HashMap::new();
    for &element in elements {
        if let Some(d) = dominants.get_mut(&element) {
            *d += 1;
        } else if dominants.len() < k {
            dominants.insert(element, 1);
        } else {
            dominants.retain(|_, v| {
                *v -= 1;
                *v > 0
            });
        }
    }

    for v in dominants.values_mut() {
        *v = 0;
    }

    for element in elements {
        if let Some(count) = dominants.get_mut(element) {
            *count += 1;
        }
    }

    let threshold = elements.len() / (k + 1);
    dominants.len() == k && dominants.values().all(|&c| c > threshold)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dominant_element_search_test() {
        assert!(dominant_element_search(&vec![2, 2, 3]));
        assert!(dominant_element_search(&vec![2, 3, 9, 2, 2]));
        assert!(!dominant_element_search(&vec![1, 2, 3, 1]));
    }

    #[test]
    fn three_dominant_element_search_test() {
        assert!(k_dominants_element_search(
            3,
            &vec![0, 9, 2, 3, 9, 0, 2, 9, 2, 3, 3]
        ));
        assert!(!k_dominants_element_search(3, &vec![1, 2, 3, 1]));
        assert!(k_dominants_element_search(3, &vec![0, 2, 1]));
        assert!(!k_dominants_element_search(
            3,
            &vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2]
        ));
    }
}
