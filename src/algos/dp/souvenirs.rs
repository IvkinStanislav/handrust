use crate::algos::dp::Cube;

pub fn souvenirs(items: Vec<usize>) -> bool {
    let sum: usize = items.iter().sum();
    if sum % 3 != 0 {
        return false;
    }
    let v = sum / 3;

    let mut cube = Cube::new_with_init_value(false, items.len() + 1, v + 1, v + 1);
    cube[(0, 0, 0)] = true;

    for i in 1..=items.len() {
        for v1 in 0..=v {
            for v2 in 0..=v {
                let item = items[i - 1];
                cube[(i, v1, v2)] = cube[(i - 1, v1, v2)];
                if v1 >= item {
                    cube[(i, v1, v2)] |= cube[(i - 1, v1 - item, v2)];
                }
                if v2 >= item {
                    cube[(i, v1, v2)] |= cube[(i - 1, v1, v2 - item)];
                }
            }
        }
    }
    cube[(items.len(), v, v)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn souvenirs_test() {
        assert_eq!(
            souvenirs(vec![1, 2, 3, 4, 5, 5, 7, 7, 8, 10, 12, 19, 25]),
            true
        );
        assert_eq!(souvenirs(vec![1, 2, 3, 4, 5, 6, 7]), false);
        assert_eq!(souvenirs(vec![12, 13, 14, 15, 23]), false);
    }
}
