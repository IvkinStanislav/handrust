pub fn booking_room(mut intervals: Vec<(usize, usize)>) -> usize {
    use std::cmp::Ordering::*;

    intervals.sort_by(|(l1, r1), (l2, r2)| match r1.cmp(r2) {
        Equal => l1.cmp(l2),
        ord => ord,
    });

    let mut result = 0;
    while let Some(&(champ_l, champ_r)) = intervals.first() {
        intervals.retain(|&(l, r)| champ_r < l || champ_l > r);
        result += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn booking_room_test() {
        assert_eq!(booking_room(vec![(1, 3), (2, 3), (4, 5)]), 2);
        assert_eq!(
            booking_room(vec![(1, 2), (2, 3), (4, 5), (4, 5), (5, 6)]),
            2
        );
        assert_eq!(booking_room(vec![(1, 50), (49, 50)]), 1);
    }
}
