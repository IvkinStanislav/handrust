use crate::algos::dp::Matrix;

pub fn max_value(expression: &str) -> isize {
    let n = expression.len() / 2;
    let mut mins = Matrix::new_with_init_value(isize::MAX, n + 1, n + 1);
    let mut maxs = Matrix::new_with_init_value(isize::MIN, n + 1, n + 1);

    for i in 0..=n {
        let a = extract_number(expression, i);
        mins[(i, i)] = a;
        maxs[(i, i)] = a;
    }

    // s = r - l => r = s + l
    for s in 1..=n {
        for l in 0..=n - s {
            let r = s + l;
            for m in l..r {
                let a = make_operation(
                    extract_operation(expression, m),
                    mins[(l, m)],
                    mins[(m + 1, r)],
                );
                let b = make_operation(
                    extract_operation(expression, m),
                    mins[(l, m)],
                    maxs[(m + 1, r)],
                );
                let c = make_operation(
                    extract_operation(expression, m),
                    maxs[(l, m)],
                    mins[(m + 1, r)],
                );
                let d = make_operation(
                    extract_operation(expression, m),
                    maxs[(l, m)],
                    maxs[(m + 1, r)],
                );
                mins[(l, r)] = mins[(l, r)].min(a).min(b).min(c).min(d);
                maxs[(l, r)] = maxs[(l, r)].max(a).max(b).max(c).max(d);
            }
        }
    }
    maxs[(0, n)]
}

fn extract_number(expression: &str, i: usize) -> isize {
    let char = expression
        .chars()
        .nth(2 * i)
        .unwrap_or_else(|| panic!("request number #{i} in {expression}"));
    char.to_digit(10)
        .unwrap_or_else(|| panic!("try convert char {char} to digit")) as isize
}

fn extract_operation(expression: &str, i: usize) -> char {
    expression
        .chars()
        .nth(2 * i + 1)
        .unwrap_or_else(|| panic!("request operation #{i} in {expression}"))
}

fn make_operation(op: char, a: isize, b: isize) -> isize {
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        op => panic!("unknown operation {op}"),
    }
}

pub fn value(expression: &str) -> isize {
    calculate(expression)
}

pub fn value_with_brackets(expression: &str) -> isize {
    calculate(expression)
}

fn calculate(expression: &str) -> isize {
    let mut stack = vec![];
    let rpn = reverse_polish_notation(expression);
    let pop2 = |stack: &mut Vec<isize>| {
        let b = stack
            .pop()
            .unwrap_or_else(|| panic!("invalid rpn from expression: {expression}"));
        let a = stack
            .pop()
            .unwrap_or_else(|| panic!("invalid rpn from expression: {expression}"));
        (a, b)
    };

    for item in rpn {
        match item {
            RpnItem::Value(value) => stack.push(value),
            RpnItem::Plus => {
                let (a, b) = pop2(&mut stack);
                stack.push(a + b);
            }
            RpnItem::Minus => {
                let (a, b) = pop2(&mut stack);
                stack.push(a - b);
            }
            RpnItem::Multiply => {
                let (a, b) = pop2(&mut stack);
                stack.push(a * b);
            }
            RpnItem::Divide => {
                let (a, b) = pop2(&mut stack);
                stack.push(a / b);
            }
            RpnItem::Degree => {
                let (a, b) = pop2(&mut stack);
                stack.push(a.pow(b as u32));
            }
        }
    }

    stack.pop().unwrap_or_default()
}

fn reverse_polish_notation(expression: &str) -> Vec<RpnItem> {
    use RpnItem::*;

    let mut stack = vec![];
    let mut ops = vec![];
    let mut value = None;
    let push_value = |value: &mut Option<isize>, stack: &mut Vec<RpnItem>| {
        if let Some(v) = value.take() {
            stack.push(Value(v));
        }
    };

    for e in expression.chars() {
        match e {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                let e = e
                    .to_digit(10)
                    .unwrap_or_else(|| panic!("try convert symbol {e} to digit"))
                    as isize;
                value = Some(value.unwrap_or_default() * 10 + e);
            }
            '+' | '-' | '*' | '/' | '^' => {
                push_value(&mut value, &mut stack);
                while let Some(&last_op) = ops.last()
                    && operation_priority(last_op) >= operation_priority(e)
                {
                    stack.push(
                        last_op
                            .try_into()
                            .unwrap_or_else(|op| panic!("try convert symbol {op:?} to operation")),
                    );
                    ops.pop();
                }
                ops.push(e);
            }
            '(' => {
                ops.push(e);
            }
            ')' => {
                push_value(&mut value, &mut stack);
                while let Some(&last_op) = ops.last()
                    && last_op != '('
                {
                    stack.push(
                        last_op
                            .try_into()
                            .unwrap_or_else(|op| panic!("try convert symbol {op:?} to operation")),
                    );
                    ops.pop();
                }
                ops.pop();
            }
            e => panic!("unknown symbol {e}"),
        }
    }

    push_value(&mut value, &mut stack);
    for op in ops.into_iter().rev() {
        stack.push(
            op.try_into()
                .unwrap_or_else(|op| panic!("try use operation {op:?} in rpn")),
        );
    }
    stack
}

fn operation_priority(op: char) -> usize {
    match op {
        '+' => 1,
        '-' => 1,
        '*' => 2,
        '/' => 2,
        '^' => 3,
        _ => 0,
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RpnItem {
    Value(isize),
    Plus,
    Minus,
    Multiply,
    Divide,
    Degree,
}

impl TryFrom<char> for RpnItem {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use RpnItem::*;

        let op = match value {
            '+' => Plus,
            '-' => Minus,
            '*' => Multiply,
            '/' => Divide,
            '^' => Degree,
            value => return Err(value),
        };

        Ok(op)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_number_test() {
        assert_eq!(extract_number("5-8+7*4-1+9", 0), 5);
        assert_eq!(extract_number("5-8+7*4-1+9", 5), 9);
        assert_eq!(extract_number("5-8+7*4-1+9", 2), 7);
    }

    #[test]
    fn extract_operation_test() {
        assert_eq!(extract_operation("5+8-7*4", 0), '+');
        assert_eq!(extract_operation("5+8-7*4", 1), '-');
        assert_eq!(extract_operation("5+8-7*4", 2), '*');
    }

    #[test]
    fn make_operation_test() {
        assert_eq!(make_operation('+', 2, 3), 5);
        assert_eq!(make_operation('-', 2, 3), -1);
        assert_eq!(make_operation('*', 2, 3), 6);
    }

    #[test]
    fn max_value_test() {
        assert_eq!(max_value("5-8+7*4-8+9"), 200);
        assert_eq!(max_value("4+1*3*2-7"), 23);
        assert_eq!(max_value("1+1+1-1*2*3+1*2-2"), 30);
    }

    #[test]
    fn reverse_polish_notation_test() {
        use RpnItem::*;
        assert_eq!(
            reverse_polish_notation("3+4*2/(1-5)^2"),
            vec![
                Value(3),
                Value(4),
                Value(2),
                Multiply,
                Value(1),
                Value(5),
                Minus,
                Value(2),
                Degree,
                Divide,
                Plus
            ]
        );
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
