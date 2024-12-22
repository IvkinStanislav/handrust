pub fn strings_sum(a: &str, b: &str) -> String {
    let mut res = Vec::with_capacity(a.len() + b.len());
    let mut a = a.chars();
    let mut b = b.chars();
    loop {
        match (a.next(), b.next()) {
            (Some(char_a), Some(char_b)) => {
                res.push(char_a);
                res.push(char_b);
            }
            (Some(char), None) => res.push(char),
            (None, Some(char)) => res.push(char),
            (None, None) => break,
        }
    }

    res.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strings_sum_test() {
        assert_eq!(strings_sum("abc", "def"), "adbecf");
        assert_eq!(strings_sum("abaca", "bdaef"), "abbdaaceaf");
        assert_eq!(strings_sum("y", "z"), "yz");
    }
}
