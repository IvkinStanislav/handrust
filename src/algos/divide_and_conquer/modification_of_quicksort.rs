/// Quick sort for a list of items with many repetitions
pub fn modification_of_quicksort(mut data: Vec<usize>) -> Vec<usize> {
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
    let (less, _, greater) = lomuto(data, target);

    quick_sort_internal(less);
    quick_sort_internal(greater);
}

fn lomuto(data: &mut [usize], target: usize) -> (&mut [usize], &mut [usize], &mut [usize]) {
    use std::cmp::Ordering::*;

    if data.len() <= 1 {
        return (data, &mut [], &mut []);
    }

    data.swap(0, target);
    let mut pointer_less = 1;
    let mut pointer_eq = 1;
    for i in 1..data.len() {
        match data[i].cmp(&data[0]) {
            Less => {
                data.swap(i, pointer_eq);
                data.swap(pointer_eq, pointer_less);
                pointer_less += 1;
                pointer_eq += 1;
            }
            Equal => {
                data.swap(i, pointer_eq);
                pointer_eq += 1;
            }
            Greater => {}
        }
    }
    data.swap(0, pointer_less - 1);
    let (less_or_eq, greater) = data.split_at_mut(pointer_eq);
    let (less, eq) = less_or_eq.split_at_mut(pointer_less - 1);
    (less, eq, greater)
}

pub fn quicksort_worst_input(n: usize) -> Vec<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn lomuto_internal_test() {
        fn test(
            mut input: Vec<usize>,
            expected_less: Vec<usize>,
            expected_eq: Vec<usize>,
            expected_greater: Vec<usize>,
        ) {
            let (less, eq, greater) = lomuto(&mut input, 0);
            assert_eq!(
                (&*less).into_iter().copied().collect::<HashSet<usize>>(),
                expected_less.into_iter().collect::<HashSet<usize>>()
            );
            assert_eq!(
                (&*eq).into_iter().copied().collect::<HashSet<usize>>(),
                expected_eq.into_iter().collect::<HashSet<usize>>()
            );
            assert_eq!(
                (&*greater).into_iter().copied().collect::<HashSet<usize>>(),
                expected_greater.into_iter().collect::<HashSet<usize>>()
            );
        }
        test(
            vec![4, 7, 2, 4, 1, 8, 4, 9],
            vec![2, 1],
            vec![4, 4, 4],
            vec![7, 8, 9],
        );
        test(
            vec![4, 7, 2, 4, 5, 3, 3, 1, 8, 4, 9, 6, 4],
            vec![2, 3, 3, 1],
            vec![4, 4, 4, 4],
            vec![7, 5, 8, 9, 6],
        );
        test(vec![3, 1, 4, 7, 3, 17], vec![1], vec![3, 3], vec![4, 7, 17]);
        test(
            vec![2, 2, 2, 3, 2, 9, 10],
            vec![],
            vec![2, 2, 2],
            vec![3, 9, 10],
        );
    }

    #[test]
    fn modification_of_quicksort_test() {
        assert_eq!(
            modification_of_quicksort(vec![2, 3, 9, 2, 2]),
            vec![2, 2, 2, 3, 9]
        );
        assert_eq!(
            modification_of_quicksort(vec![1, 2, 3, 1]),
            vec![1, 1, 2, 3]
        );
    }

    #[test]
    fn quicksort_worst_input_test() {
        assert_eq!(quicksort_worst_input(2), vec![2, 1]);
        assert_eq!(quicksort_worst_input(4), vec![2, 4, 1, 3]);
    }
}
