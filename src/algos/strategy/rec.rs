pub fn towers3_full(n: usize) -> Vec<(usize, usize)> {
    towers3_inner_full(n, 1, 3)
}

fn towers3_inner_full(n: usize, from: usize, to: usize) -> Vec<(usize, usize)> {
    if n == 1 {
        vec![(from, to)]
    } else {
        let mut res = vec![];
        let mid = 6 - from - to;
        res.append(&mut towers3_inner_full(n - 1, from, mid));
        res.push((from, to));
        res.append(&mut towers3_inner_full(n - 1, mid, to));
        res
    }
}

pub fn towers4(n: usize) -> usize {
    // Алгоритм Фрейма — Стюарта
    if n == 1 {
        1
    } else {
        let k = n / 2;
        2 * towers4(k) + towers3(n - k)
    }
}

fn towers3(n: usize) -> usize {
    if n == 1 {
        1
    } else {
        towers3(n - 1) + 1 + towers3(n - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn towers3_test() {
        assert_eq!(
            towers3_full(3),
            vec![(1, 3), (1, 2), (3, 2), (1, 3), (2, 1), (2, 3), (1, 3)]
        );
        assert_eq!(
            towers3_full(4),
            vec![
                (1, 2),
                (1, 3),
                (2, 3),
                (1, 2),
                (3, 1),
                (3, 2),
                (1, 2),
                (1, 3),
                (2, 3),
                (2, 1),
                (3, 1),
                (2, 3),
                (1, 2),
                (1, 3),
                (2, 3)
            ]
        );
        assert_eq!(
            towers3_full(5),
            vec![
                (1, 3),
                (1, 2),
                (3, 2),
                (1, 3),
                (2, 1),
                (2, 3),
                (1, 3),
                (1, 2),
                (3, 2),
                (3, 1),
                (2, 1),
                (3, 2),
                (1, 3),
                (1, 2),
                (3, 2),
                (1, 3),
                (2, 1),
                (2, 3),
                (1, 3),
                (2, 1),
                (3, 2),
                (3, 1),
                (2, 1),
                (2, 3),
                (1, 3),
                (1, 2),
                (3, 2),
                (1, 3),
                (2, 1),
                (2, 3),
                (1, 3)
            ]
        );
    }

    #[test]
    fn towers4_test() {
        assert_eq!(towers4(3), 5);
        assert_eq!(towers4(4), 9);
        assert_eq!(towers4(5), 13);
    }
}
