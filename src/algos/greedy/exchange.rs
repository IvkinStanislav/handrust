use std::collections::HashSet;

pub fn exchange_variants(n: usize) -> HashSet<Vec<usize>> {
    let mut acc = HashSet::new();
    exchange_variants_internal_1_5_10(counts_of_1_5_10(n), &mut acc);

    let mut res = HashSet::new();
    for counts in acc {
        res.insert([vec![1; counts[0]], vec![5; counts[1]], vec![10; counts[2]]].concat());
    }
    res
}

/// [count_of_1, count_of_5, count_of_10]
fn counts_of_1_5_10(n: usize) -> [usize; 3] {
    [n % 5, (n % 10) / 5, n / 10]
}

fn exchange_variants_internal_1_5_10(counts: [usize; 3], acc: &mut HashSet<[usize; 3]>) {
    acc.insert(counts);
    let [count_of_1, count_of_5, count_of_10] = counts;

    if counts[2] > 0 {
        let new_counts = [count_of_1, count_of_5 + 2, count_of_10 - 1];
        acc.insert(new_counts);
        exchange_variants_internal_1_5_10(new_counts, acc);
    }

    if counts[1] > 0 {
        let new_counts = [count_of_1 + 5, count_of_5 - 1, count_of_10];
        acc.insert(new_counts);
        exchange_variants_internal_1_5_10(new_counts, acc);
    }
}

pub fn exchange_1_5_10_20_50(n: usize) -> Vec<usize> {
    let counts = counts_of_1_5_10_20_50(n);
    [
        vec![1; counts[0]],
        vec![5; counts[1]],
        vec![10; counts[2]],
        vec![20; counts[3]],
        vec![50; counts[4]],
    ]
    .concat()
}

/// [count_of_1, count_of_5, count_of_10, count_of_20, count_of_50]
fn counts_of_1_5_10_20_50(n: usize) -> [usize; 5] {
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
    fn exchange_variants_test() {
        assert_eq!(
            exchange_variants(15),
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
            exchange_variants(10),
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
            exchange_variants(1),
            {
                let mut set = HashSet::new();
                set.insert(vec![1]);
                set
            }
        );
        assert_eq!(
            exchange_variants(5),
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
