pub fn nearest_points_pair(mut points: Vec<(isize, isize)>) -> f64 {
    points.sort_by(|(x1, _), (x2, _)| x1.cmp(x2));
    nearest_points_pair_internal(&points)
}

fn nearest_points_pair_internal(points: &[(isize, isize)]) -> f64 {
    match points.len() {
        0 | 1 => return f64::MAX,
        2 => return distance(points[0], points[1]),
        _ => {}
    }

    let (left, right) = points.split_at(points.len() / 2);
    let border_line = left[left.len() - 1].0 + right[0].0 / 2;

    let d1 = nearest_points_pair_internal(left);
    let d2 = nearest_points_pair_internal(right);
    let mut d = d1.min(d2);

    let (low, high) = (
        border_line - d.ceil() as isize,
        border_line + d.ceil() as isize,
    );
    let mut filtered_points: Vec<_> = points
        .iter()
        .filter(|(x, _)| *x >= low && *x <= high)
        .collect();
    filtered_points.sort_by(|(_, y1), (_, y2)| y1.cmp(y2));

    for i in 0..filtered_points.len() {
        for j in i + 1..i + 8 {
            if j < filtered_points.len() {
                d = d.min(distance(*filtered_points[i], *filtered_points[j]))
            } else {
                break;
            }
        }
    }

    d
}

fn distance((x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> f64 {
    f64::sqrt((x2 - x1).pow(2) as f64 + (y2 - y1).pow(2) as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nearest_points_pair_test() {
        nearest_points_pair_test_helper(vec![(0, 0), (3, 4)]);
        nearest_points_pair_test_helper(vec![
            (4, 4),
            (-2, -2),
            (-3, -4),
            (-1, 3),
            (2, 3),
            (-4, 0),
            (1, 1),
            (-1, -1),
            (3, -1),
            (-4, 2),
            (-2, 4),
        ]);
        nearest_points_pair_test_helper(vec![(0, 0), (1, 1), (0, 0)]);
    }

    fn nearest_points_pair_test_helper(points: Vec<(isize, isize)>) {
        assert_eq!(
            nearest_points_pair(points.clone()),
            bad_nearest_points_pair(points)
        )
    }

    fn bad_nearest_points_pair(points: Vec<(isize, isize)>) -> f64 {
        let mut d = f64::MAX;
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                d = d.min(distance(points[i], points[j]));
            }
        }
        d
    }
}
