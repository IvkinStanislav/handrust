use super::Matrix;

pub fn editing_distance_rec(s1: &str, s2: &str) -> usize {
    let s1: Vec<_> = s1.chars().collect();
    let s2: Vec<_> = s2.chars().collect();
    let mut dp = Matrix::new_with_init_value(usize::MAX, s1.len(), s2.len());
    editing_distance_rec_internal(&s1, &s2, &mut dp)
}

fn editing_distance_rec_internal(s1: &[char], s2: &[char], dp: &mut Matrix<usize>) -> usize {
    let Some((s1_last, s1_remainder)) = s1.split_last() else {
        return s2.len();
    };
    let Some((s2_last, s2_remainder)) = s2.split_last() else {
        return s1.len();
    };

    let (i, j) = (s1.len() - 1, s2.len() - 1);
    if dp[i][j] != usize::MAX {
        return dp[i][j];
    }

    let deletion = editing_distance_rec_internal(s1_remainder, s2, dp) + 1;
    let insertion = editing_distance_rec_internal(s1, s2_remainder, dp) + 1;
    let replacement = editing_distance_rec_internal(s1_remainder, s2_remainder, dp)
        + if s1_last == s2_last { 0 } else { 1 };

    dp[i][j] = replacement.min(insertion).min(deletion);
    dp[i][j]
}

pub fn editing_distance(s1: &str, s2: &str) -> usize {
    let s1: Vec<_> = s1.chars().collect();
    let s2: Vec<_> = s2.chars().collect();
    let dp = create_matrix(&s1, &s2);
    dp[s1.len()][s2.len()]
}

fn create_matrix(s1: &[char], s2: &[char]) -> Matrix<usize> {
    let mut dp = Matrix::new_with_init_value(usize::MAX, s1.len() + 1, s2.len() + 1);

    for i in 0..=s1.len() {
        dp[i][0] = i;
    }
    for j in 0..=s2.len() {
        dp[0][j] = j;
    }
    for i in 1..=s1.len() {
        for j in 1..=s2.len() {
            let deletion = dp[i - 1][j] + 1;
            let insertion = dp[i][j - 1] + 1;
            let replacement = dp[i - 1][j - 1] + if s1[i - 1] == s2[j - 1] { 0 } else { 1 };
            dp[i][j] = replacement.min(insertion).min(deletion);
        }
    }
    dp
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum StringOperation {
    Deletion,
    Insertion,
    Replacement,
    Nothing,
}

pub fn editing_distance_detailed(s1: &str, s2: &str) -> Vec<String> {
    use StringOperation::*;

    let mut s1: Vec<_> = s1.chars().collect();
    let s2: Vec<_> = s2.chars().collect();
    let dp = create_matrix(&s1, &s2);

    let mut operations = Vec::with_capacity(dp[s1.len()][s2.len()]);
    let (mut i, mut j) = (s1.len(), s2.len());

    while i > 0 && j > 0 {
        let deletion = dp[i - 1][j];
        let insertion = dp[i][j - 1];
        let replacement = dp[i - 1][j - 1];
        let save_in_replacement = replacement == dp[i][j];
        let min = replacement.min(insertion).min(deletion);

        let operation = if min == replacement {
            i -= 1;
            j -= 1;
            if save_in_replacement {
                Nothing
            } else {
                Replacement
            }
        } else if min == insertion {
            j -= 1;
            Insertion
        } else if min == deletion {
            i -= 1;
            Deletion
        } else {
            unreachable!()
        };
        operations.push(operation);
    }
    for _ in 0..i {
        operations.push(Deletion);
    }
    for _ in 0..j {
        operations.push(Insertion);
    }
    operations.reverse();

    let (mut i, mut j) = (0, 0);
    let mut steps = vec![s1.iter().copied().collect()];
    for operation in operations {
        match operation {
            Nothing => {
                i += 1;
                j += 1;
            }
            Replacement => {
                s1[i] = s2[j];
                i += 1;
                j += 1;
            }
            Deletion => {
                s1.remove(i);
            }
            Insertion => {
                s1.insert(i, s2[j]);
                i += 1;
                j += 1;
            }
        };
        if operation != Nothing {
            steps.push(s1.iter().copied().collect());
        }
    }
    steps
}

pub fn editing_distance_improved(s1: &str, s2: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn editing_distance_test_internal(s1: &str, s2: &str, steps: Vec<&str>) {
        assert_eq!(editing_distance_detailed(s1, s2), steps);
        assert_eq!(editing_distance_rec(s1, s2), steps.len() - 1);
        assert_eq!(editing_distance(s1, s2), steps.len() - 1);
        assert_eq!(editing_distance_improved(s1, s2), steps.len() - 1);
    }

    #[test]
    fn editing_distance_test() {
        //     p o r t s
        //   0 1 2 3 4 5
        // s 1 1 2 3 4 4
        // h 2 2 2 3 4 5
        // o 3 3 2 3 4 5
        // r 4 4 3 2 3 4
        // t 5 5 4 3 2 3
        editing_distance_test_internal("short", "ports", vec!["short", "hort", "port", "ports"]);
        // editing_distance_test_internal("a", "a", vec!["a"]);
        // editing_distance_test_internal("ada", "aba", vec!["ada", "aba"]);
        // editing_distance_test_internal("abacab", "bacacaba", vec!["abacab", "babacab", "bacacab", "bacacaba"]);
        // editing_distance_test_internal("aaaaaac", "caaaaaa", vec!["aaaaaac", "caaaaac", "caaaaaa"]);
    }
}
