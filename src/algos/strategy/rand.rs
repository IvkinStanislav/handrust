pub fn lomuto(mut data: Vec<usize>) -> Vec<usize> {
    lomuto_internal(&mut data, 0);
    data
}

pub fn quick_sort(mut data: Vec<usize>) -> Vec<usize> {
    quick_sort_internal(&mut data);
    data
}

fn lomuto_internal(data: &mut [usize], target: usize) -> usize {
    if data.len() <= 1 {
        return 0;
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
    pointer - 1
}

// fn lomuto_internal(data: &mut [usize], mut target: usize) -> usize {
//     if data.len() <= 1 {
//         return 0;
//     }

//     let mut pointer = 0;
//     for i in 0..data.len() {
//         if data[i] <= data[target] {
//             data.swap(i, pointer);
//             if i == target {
//                 target = pointer
//             }
//             pointer += 1;
//         }
//     }
//     data.swap(target, pointer - 1);
//     pointer - 1
// }

fn quick_sort_internal(data: &mut [usize]) {
    if data.len() <= 2 {
        if data.len() == 2 && data[0] > data[1] {
            data.swap(0, 1);
        }
        return;
    }

    let target = rand::random::<usize>() % data.len();
    let mid = lomuto_internal(data, target);
    let (left, right) = data.split_at_mut(mid);

    quick_sort_internal(left);
    quick_sort_internal(right);
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
