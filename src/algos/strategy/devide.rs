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

pub fn merge(data: Vec<Vec<usize>>) -> Vec<usize> {
    let mut res = vec![];
    let mut data: Vec<_> = data.into_iter().map(|d| d.into_iter().peekable()).collect();

    loop {
        let min = data
            .iter_mut()
            .filter_map(|iter| {
                if let Some(&item) = iter.peek() {
                    Some((iter, item))
                } else {
                    None
                }
            })
            .min_by(|(_, item1), (_, item2)| item1.cmp(item2))
            .and_then(|(iter, _)| iter.next());

        if let Some(min) = min {
            res.push(min);
        } else {
            break;
        }
    }

    res
}

pub fn merge_sort(data: Vec<usize>) -> Vec<usize> {
    if data.len() <= 1 {
        return data;
    }

    let left = merge_sort(data[..data.len() / 2].to_vec());
    let right = merge_sort(data[data.len() / 2..].to_vec());
    merge(vec![left, right])
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

    #[test]
    fn merge_sort_test() {
        assert_eq!(
            merge_sort(vec![13, 17, 37, 73, 31, 19, 23]),
            vec![13, 17, 19, 23, 31, 37, 73]
        );
        assert_eq!(merge_sort(vec![18, 20, 3, 17]), vec![3, 17, 18, 20]);
        assert_eq!(merge_sort(vec![0, 11, 0]), vec![0, 0, 11]);
    }
}
