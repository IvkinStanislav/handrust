use crate::algos::dp::Matrix;

pub fn max_value(expression: &str) -> isize {
    let n = expression.len() / 2;
    let mut mins = Matrix::new_with_init_value(isize::MAX, n + 1, n + 1);
    let mut maxs = Matrix::new_with_init_value(isize::MIN, n + 1, n + 1);

    for i in 0..=n {
        let a = number_by_index(expression, i);
        mins[(i, i)] = a;
        maxs[(i, i)] = a;
    }

    // s = r - l => r = s + l
    for s in 1..=n {
        for l in 0..=n - s {
            let r = s + l;
            for m in l..r {
                let a = operation(mins[(l, m)], mins[(m + 1, r)], expression, m);
                let b = operation(mins[(l, m)], maxs[(m + 1, r)], expression, m);
                let c = operation(maxs[(l, m)], mins[(m + 1, r)], expression, m);
                let d = operation(maxs[(l, m)], maxs[(m + 1, r)], expression, m);
                mins[(l, r)] = mins[(l, r)].min(a).min(b).min(c).min(d);
                maxs[(l, r)] = maxs[(l, r)].max(a).max(b).max(c).max(d);
            }
        }
    }
    maxs[(0, n)]
}

fn number_by_index(expression: &str, i: usize) -> isize {
    let char = expression
        .chars()
        .nth(2 * i)
        .unwrap_or_else(|| panic!("request number #{i} in {expression}"));
    char.to_digit(10)
        .unwrap_or_else(|| panic!("try convert char {char} to digit")) as isize
}

fn operation(a: isize, b: isize, expression: &str, i: usize) -> isize {
    let op = expression
        .chars()
        .nth(2 * i + 1)
        .unwrap_or_else(|| panic!("request operation #{i} in {expression}"));
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        op => panic!("unknown operation {op}"),
    }
}

pub fn value(expression: &str) -> isize {
    todo!()
}

pub fn value_with_brackets(expression: &str) -> isize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_by_index_test() {
        assert_eq!(number_by_index("5-8+7*4-1+9", 0), 5);
        assert_eq!(number_by_index("5-8+7*4-1+9", 5), 9);
        assert_eq!(number_by_index("5-8+7*4-1+9", 2), 7);
    }

    #[test]
    fn operation_test() {
        assert_eq!(operation(2, 3, "5+8-7*4", 0), 5);
        assert_eq!(operation(2, 3, "5+8-7*4", 1), -1);
        assert_eq!(operation(2, 3, "5+8-7*4", 2), 6);
    }

    #[test]
    fn max_value_test() {
        assert_eq!(max_value("5-8+7*4-8+9"), 200);
        assert_eq!(max_value("4+1*3*2-7"), 23);
        assert_eq!(max_value("1+1+1-1*2*3+1*2-2"), 30);
    }

    #[test]
    fn value_test() {
        assert_eq!(value("5-8+7*4-8+9"), 26);
        assert_eq!(value("23*17-18+37"), 410);
        assert_eq!(value("1+1+4+0-0*0+15"), 21);
    }

    #[test]
    fn value_with_brackets_test() {
        assert_eq!(value_with_brackets("(1+2)*4"), 12);
        assert_eq!(value_with_brackets("(2+2)*2+0*1-10"), -2);
        assert_eq!(value_with_brackets("((2+2)*2)"), 8);
    }
}
