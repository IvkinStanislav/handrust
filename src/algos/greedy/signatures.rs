pub fn collection_of_signatures(mut coordinates: Vec<(usize, usize)>) -> Vec<usize> {
    use std::cmp::Ordering::*;

    coordinates.sort_by(|(l1, r1), (l2, r2)| match r1.cmp(r2) {
        Equal => l1.cmp(l2),
        ord => ord,
    });

    let mut result = vec![];
    while let Some(&(champ_l, champ_r)) = coordinates.first() {
        coordinates.retain(|&(l, r)| champ_r < l || champ_l > r);
        result.push(champ_r);
    }

    result
}

pub fn covering(l: usize, mut coordinates: Vec<usize>) -> usize {
    let mut sum_segments;
    let mut next_segment;
    coordinates.sort_unstable();

    if let Some(first) = coordinates.first() {
        sum_segments = 1;
        next_segment = first + l;
    } else {
        return 0;
    }

    for x in coordinates.into_iter().skip(1) {
        if x > next_segment {
            next_segment = x + l;
            sum_segments += 1;
        }
    }

    sum_segments
}

pub fn min_segments_sum(k: usize, coordinates: Vec<usize>) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn collection_of_signatures_test() {
        assert_eq!(collection_of_signatures(vec![(1, 3), (2, 5), (3, 6)]), vec![3]);
        assert_eq!(collection_of_signatures(vec![(4, 7), (1, 3), (2, 5), (3, 6)]), vec![3, 7]);
    }

    #[test]
    fn covering_test() {
        assert_eq!(covering(4, vec![1, 1, 3, 4, 5]), 1);
        assert_eq!(covering(1, vec![1, 2, 3, 4, 5]), 3);
    }

    #[test]
    fn min_segments_sum_test() {
        assert_eq!(min_segments_sum(2, vec![1, 1, 3, 4, 5]), 2);
        assert_eq!(min_segments_sum(1000000000, vec![1, 1, 3, 4, 5]), 0);
        assert_eq!(
            min_segments_sum(3, vec![1, 4, 2, 10, 20, 11, 12, 14, 19, 15]),
            9
        );
    }
}
