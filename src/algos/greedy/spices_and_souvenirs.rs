pub fn spices(mut max_weight: usize, counts: Vec<(usize, usize)>) -> f64 {
    let mut counts: Vec<_> = counts
        .iter()
        .map(|&(c, w)| (c, w, c as f64 / w as f64))
        .collect();
    counts.sort_by(|(_, _, cpw1), (_, _, cpw2)| cpw2.total_cmp(cpw1));

    let mut sum = 0.0;
    for (cost, weight, cost_per_weight) in counts {
        if max_weight > weight {
            sum += cost as f64;
            max_weight -= weight;
        } else {
            sum += cost_per_weight * max_weight as f64;
            break;
        }
    }

    sum
}

pub fn souvenirs(mut money: usize, mut costs: Vec<usize>) -> usize {
    costs.sort();
    let mut sum = 0;
    for cost in costs {
        if money >= cost {
            sum += 1;
            money -= cost;
        } else {
            break;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::algos::f64_round;

    use super::*;

    #[test]
    #[rustfmt::skip]
    fn spices_test() {
        assert_eq!(spices_test_helper(50, vec![(60, 20), (100, 50), (120, 30)]), 180.000);
        assert_eq!(spices_test_helper(10, vec![(500, 30)]), 166.667);
        assert_eq!(spices_test_helper(1000, vec![(500, 30)]), 500.000);
    }

    #[test]
    fn souvenirs_test() {
        assert_eq!(souvenirs(50, vec![20, 50, 30]), 2);
        assert_eq!(souvenirs(1, vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1]), 6);
        assert_eq!(souvenirs(10, vec![500]), 0);
    }

    fn spices_test_helper(max_weight: usize, counts: Vec<(usize, usize)>) -> f64 {
        f64_round(spices(max_weight, counts), 3)
    }
}
