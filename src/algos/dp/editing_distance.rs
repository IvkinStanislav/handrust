use super::Matrix;

pub fn editing_distance(s1: &str, s2: &str) -> usize {
    let s1: Vec<_> = s1.chars().collect();
    let s2: Vec<_> = s2.chars().collect();
    let mut dp = Matrix::new_with_init_value(usize::MAX, s1.len() + 1, s2.len() + 1);
    for i in 0..=s1.len() {
        dp[i][0] = i;
    }
    for j in 0..=s2.len() {
        dp[0][j] = j;
    }
    editing_distance_internal(&s1, &s2, &mut dp)
}

fn editing_distance_internal(s1: &[char], s2: &[char], dp: &mut Matrix<usize>) -> usize {
    for i in 1..=s1.len() {
        for j in 1..=s2.len() {
            let deletion = dp[i - 1][j] + 1;
            let insertion = dp[i][j - 1] + 1;
            let replacement = dp[i - 1][j - 1] + if s1[i - 1] == s2[j - 1] { 0 } else { 1 };
            dp[i][j] = replacement.min(insertion).min(deletion);
        }
    }
    dp[s1.len()][s2.len()]
}

pub fn editing_distance_detailed(s1: &str, s2: &str) -> Vec<String> {
    todo!()
}

pub fn editing_distance_improved(s1: &str, s2: &str) -> usize {
    todo!()
}

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

#[cfg(test)]
mod tests {
    use super::*;

    fn editing_distance_test_internal(s1: &str, s2: &str, expected: usize) {
        assert_eq!(editing_distance(s1, s2), expected);
        assert_eq!(editing_distance_rec(s1, s2), expected);
    }

    #[test]
    fn editing_distance_test() {
        editing_distance_test_internal("abacab", "bacacaba", 3);
        editing_distance_test_internal("aaaaaac", "caaaaaa", 2);
        editing_distance_test_internal("ada", "aba", 1)
    }
}
