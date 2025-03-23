use crate::algos::dp::Matrix;

pub fn backpack(w: usize, items: Vec<usize>) -> usize {
    let mut dp = Matrix::new_with_init_value(false, w + 1, items.len() + 1);
    dp[(0, 0)] = true;

    for i in 0..=w {
        for j in 1..=items.len() {
            if items[j - 1] > i {
                dp[(i, j)] = dp[(i, j - 1)]
            } else {
                dp[(i, j)] = dp[(i, j - 1)] || dp[(i - items[j - 1], j - 1)]
            }
        }
    }

    for i in (0..=w).rev() {
        for j in (0..=items.len()).rev() {
            if dp[(i, j)] {
                return i;
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backpack_test() {
        assert_eq!(backpack(10, vec![1, 4, 9]), 10);
        assert_eq!(backpack(73, vec![19, 23, 31, 17, 18]), 73);
        assert_eq!(backpack(44, vec![44, 43, 1]), 44);
    }
}
