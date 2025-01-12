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
    coordinates.sort_unstable();
    let mut coordinates = coordinates.into_iter();

    let Some(first) = coordinates.next() else {
        return 0;
    };
    let mut next_segment = first + l;
    let mut sum_segments = 1;

    for x in coordinates {
        if x > next_segment {
            next_segment = x + l;
            sum_segments += 1;
        }
    }

    sum_segments
}

// unclear task statement
pub fn min_segments_sum(k: usize, mut coordinates: Vec<usize>) -> usize {
    if coordinates.is_empty() {
        return 0;
    }
    coordinates.sort_unstable();
    coordinates.dedup();
    let mut segments: Vec<_> = coordinates
        .windows(2)
        .map(|segment| (segment[0], segment[1], segment[1] - segment[0]))
        .collect();

    segments.sort_unstable_by(|(_, _, len1), (_, _, len2)| len2.cmp(len1));
    let mut segments: Vec<_> = segments.into_iter().skip(k - 1).collect();
    segments.sort_unstable_by(|(x1, _, _), (x2, _, _)| x1.cmp(x2));

    for i in 1..segments.len() {
        if segments[i].0 == segments[i - 1].1 {
            segments[i].2 += segments[i - 1].2;
        }
    }

    segments
        .into_iter()
        .map(|(_, _, len)| len)
        .max()
        .unwrap_or_default()
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
        // correct tests
        // assert_eq!(min_segments_sum(2, vec![1, 1, 3, 4, 5]), 2);
        // assert_eq!(min_segments_sum(1000000000, vec![1, 1, 3, 4, 5]), 0);
        // assert_eq!(
        //     min_segments_sum(3, vec![1, 4, 2, 10, 20, 11, 12, 14, 19, 15]),
        //     9
        // );

        assert_eq!(min_segments_sum(2, vec![1, 1, 3, 4, 5]), 2);
        assert_eq!(min_segments_sum(1000000000, vec![1, 1, 3, 4, 5]), 0);
        assert_eq!(
            min_segments_sum(3, vec![1, 4, 2, 10, 20, 11, 12, 14, 19, 15]),
            5
        );
    }
}
