use super::Matrix;

pub fn editing_distance(s1: &str, s2: &str) -> usize {
    let s1: Vec<_> = s1.chars().collect();
    let s2: Vec<_> = s2.chars().collect();
    let mut dp = Matrix::new_with_init_value(usize::MAX, s1.len() + 1, s2.len() + 1);
    editing_distance_internal(&s1, &s2, &mut dp)
}

fn editing_distance_internal(s1: &[char], s2: &[char], dp: &mut Matrix<usize>) -> usize {
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

    let deletion = editing_distance_internal(s1_remainder, s2, dp) + 1;
    let insertion = editing_distance_internal(s1, s2_remainder, dp) + 1;
    let replacement = editing_distance_internal(s1_remainder, s2_remainder, dp)
        + (s1_last == s2_last).then_some(0).unwrap_or(1);

    dp[i][j] = replacement.min(insertion).min(deletion);
    dp[i][j]
}

#[allow(dead_code)]
fn editing_distance_rec(s1: &str, s2: &str) -> usize {
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
        + (s1_last == s2_last).then_some(0).unwrap_or(1);

    dp[i][j] = replacement.min(insertion).min(deletion);
    dp[i][j]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn editing_distance_test() {
        assert_eq!(editing_distance("abacab", "bacacaba"), 3);
        assert_eq!(editing_distance("aaaaaac", "caaaaaa"), 2);
        assert_eq!(editing_distance("ada", "aba"), 1);
    }
}
