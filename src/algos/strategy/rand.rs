pub fn lomuto(mut data: Vec<usize>) -> Vec<usize> {
    lomuto_internal(&mut data, 0);
    data
}

pub fn quick_sort(mut data: Vec<usize>) -> Vec<usize> {
    quick_sort_internal(&mut data);
    data
}

fn quick_sort_internal(data: &mut [usize]) {
    if data.len() <= 2 {
        if data.len() == 2 && data[0] > data[1] {
            data.swap(0, 1);
        }
        return;
    }

    let target = rand::random::<usize>() % data.len();
    let (left, right) = lomuto_internal(data, target);

    quick_sort_internal(left);
    quick_sort_internal(right);
}

fn lomuto_internal(data: &mut [usize], target: usize) -> (&mut [usize], &mut [usize]) {
    if data.len() <= 1 {
        return (data, &mut []);
    }

    data.swap(0, target);
    let mut pointer = 1;
    for i in 1..data.len() {
        if data[i] <= data[0] {
            data.swap(i, pointer);
            pointer += 1;
        }
    }
    data.swap(0, pointer - 1);
    data.split_at_mut(pointer - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lomuto_test() {
        assert_eq!(
            lomuto(vec![4, 7, 2, 5, 3, 1, 8, 9, 6]),
            vec![1, 2, 3, 4, 7, 5, 8, 9, 6]
        );
        assert_eq!(lomuto(vec![3, 4, 7, 17]), vec![3, 4, 7, 17]);
        assert_eq!(lomuto(vec![1, 3, 2, 9, 10]), vec![1, 3, 2, 9, 10]);
    }

    #[test]
    fn quick_sort_test() {
        assert_eq!(
            quick_sort(vec![13, 17, 37, 73, 31, 19, 23]),
            vec![13, 17, 19, 23, 31, 37, 73]
        );
        assert_eq!(quick_sort(vec![18, 20, 3, 17]), vec![3, 17, 18, 20]);
        assert_eq!(quick_sort(vec![1, 11, 1]), vec![1, 1, 11]);
    }
}
