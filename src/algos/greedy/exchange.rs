use std::collections::HashSet;

pub fn exchange(n: usize) -> HashSet<Vec<usize>> {
    let mut acc = HashSet::new();
    exchange_internal(get_values(n), &mut acc);

    let mut res = HashSet::new();
    for values in acc {
        res.insert([vec![1; values[0]], vec![5; values[1]], vec![10; values[2]]].concat());
    }
    res
}

fn get_values(n: usize) -> [usize; 3] {
    [
        n % 5,
        (n % 10) / 5,
        n / 10,
    ]
}

// acc = [count_of_1, count_of_5, count_of_10]
fn exchange_internal(values: [usize; 3], acc: &mut HashSet<[usize; 3]>) {
    if values[2] > 0 {
        let mut values = values.clone();
        values[2] -= 1;
        values[1] += 2;
        acc.insert(values);
        exchange_internal(values, acc);
    }

    if values[1] > 0 {
        let mut values = values.clone();
        values[1] -= 1;
        values[0] += 5;
        acc.insert(values);
        exchange_internal(values, acc);
    }

    acc.insert(values);
}

pub fn exchange_1_5_10_20_50(n: usize) -> Vec<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn exchange_test() {
        assert_eq!(
            exchange(15),
            {
                let mut set = HashSet::new();
                set.insert(vec![5, 10]);
                set.insert(vec![5, 5, 5]);
                set.insert(vec![1, 1, 1, 1, 1, 10]);
                set.insert(vec![1, 1, 1, 1, 1, 5, 5]);
                set.insert(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5]);
                set.insert(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
                set
            }
        );
        assert_eq!(
            exchange(10),
            {
                let mut set = HashSet::new();
                set.insert(vec![10]);
                set.insert(vec![5, 5]);
                set.insert(vec![1, 1, 1, 1, 1, 5]);
                set.insert(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
                set
            }
        );
        assert_eq!(
            exchange(1),
            {
                let mut set = HashSet::new();
                set.insert(vec![1]);
                set
            }
        );
        assert_eq!(
            exchange(5),
            {
                let mut set = HashSet::new();
                set.insert(vec![5]);
                set.insert(vec![1, 1, 1, 1, 1]);
                set
            }
        );
    }

    #[test]
    fn exchange_1_5_10_20_50_test() {
        assert_eq!(exchange_1_5_10_20_50(7), vec![5, 1, 1]);
        assert_eq!(exchange_1_5_10_20_50(10), vec![10]);
        assert_eq!(exchange_1_5_10_20_50(10), vec![1]);
    }
}
