use std::collections::HashSet;

pub fn exchange(n: usize) -> HashSet<Vec<usize>> {
    let mut acc = HashSet::new();
    exchange_internal(get_values_1_5_10(n), &mut acc);

    let mut res = HashSet::new();
    for values in acc {
        res.insert([vec![1; values[0]], vec![5; values[1]], vec![10; values[2]]].concat());
    }
    res
}

fn get_values_1_5_10(n: usize) -> [usize; 3] {
    [n % 5, (n % 10) / 5, n / 10]
}

// acc = [count_of_1, count_of_5, count_of_10]
fn exchange_internal(values: [usize; 3], acc: &mut HashSet<[usize; 3]>) {
    if values[2] > 0 {
        let mut vs = values;
        vs[2] -= 1;
        vs[1] += 2;
        acc.insert(vs);
        exchange_internal(vs, acc);
    }

    if values[1] > 0 {
        let mut vs = values;
        vs[1] -= 1;
        vs[0] += 5;
        acc.insert(vs);
        exchange_internal(vs, acc);
    }

    acc.insert(values);
}

pub fn exchange_1_5_10_20_50(n: usize) -> Vec<usize> {
    let values = get_values_1_5_10_20_50(n);
    [
        vec![1; values[0]],
        vec![5; values[1]],
        vec![10; values[2]],
        vec![20; values[3]],
        vec![50; values[4]],
    ]
    .concat()
}

fn get_values_1_5_10_20_50(n: usize) -> [usize; 5] {
    [
        n % 5,
        (n % 10) / 5,
        (n % 50 % 20) / 10,
        (n % 50) / 20,
        n / 50,
    ]
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
        assert_eq!(exchange_1_5_10_20_50(7), vec![1, 1, 5]);
        assert_eq!(exchange_1_5_10_20_50(10), vec![10]);
        assert_eq!(exchange_1_5_10_20_50(1), vec![1]);
    }
}
