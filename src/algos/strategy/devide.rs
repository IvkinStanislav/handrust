pub fn select_sort(mut data: Vec<usize>) -> Vec<usize> {
    if data.len() > 1 {
        for i in 0..data.len() {
            let min_i = data
                .iter()
                .enumerate()
                .skip(i)
                .min_by(|(_, e1), (_, e2)| e1.cmp(e2))
                .map(|(i, _)| i);

            if let Some(min_i) = min_i {
                data.swap(i, min_i);
            }
        }
    }

    data
}

// переписать, создание data.iter_mut() - лишнее
pub fn merge(data: Vec<Vec<usize>>) -> Vec<usize> {
    let mut res = vec![];
    let mut data: Vec<_> = data
        .into_iter()
        .filter(|d| !d.is_empty())
        .map(|d| d.into_iter().peekable())
        .collect();

    if data.is_empty() {
        return vec![];
    }

    loop {
        let mut min_and_target = None;

        for (i, array) in data.iter_mut().enumerate() {
            let Some(&e) = array.peek() else {
                continue;
            };

            if let Some((min, _)) = min_and_target {
                if e < min {
                    min_and_target = Some((e, i));
                }
            } else {
                min_and_target = Some((e, i));
            }
        }

        if let Some((min, target)) = min_and_target {
            res.push(min);
            data[target].next();
        } else {
            break;
        }
    }

    res
}

pub fn merge_sort(data: Vec<usize>) -> Vec<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_sort_test() {
        assert_eq!(
            select_sort(vec![13, 17, 37, 73, 31, 19, 23]),
            vec![13, 17, 19, 23, 31, 37, 73]
        );
        assert_eq!(
            select_sort(vec![12, 18, 7, 11, 5, 17]),
            vec![5, 7, 11, 12, 17, 18]
        );
        assert_eq!(select_sort(vec![1, 2, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn merge_test() {
        assert_eq!(
            merge(vec![vec![1, 2, 3], vec![1, 2], vec![3, 4, 5, 6]]),
            vec![1, 1, 2, 2, 3, 3, 4, 5, 6]
        );
        assert_eq!(
            merge(vec![vec![1, 10], vec![7, 9, 11]]),
            vec![1, 7, 9, 10, 11]
        );
        assert_eq!(
            merge(vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]]),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        );
    }

    // #[test]
    // fn merge_sort_test() {
    //     assert_eq!(
    //         merge_sort(vec![13, 17, 37, 73, 31, 19, 23]),
    //         vec![13, 17, 19, 23, 31, 37, 73]
    //     );
    //     assert_eq!(merge_sort(vec![18, 20, 3, 17]), vec![3, 17, 18, 20]);
    //     assert_eq!(merge_sort(vec![0, 11, 0]), vec![0, 0, 11]);
    // }
}
