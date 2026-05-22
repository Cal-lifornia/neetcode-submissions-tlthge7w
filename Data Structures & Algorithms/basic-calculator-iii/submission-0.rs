impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut s = s;
        s.push('@');
        let mut chars = s.chars();
        calculate_expression(&mut chars)
    }
}

fn calculate_expression(chars: &mut std::str::Chars) -> i32 {
    let mut prev_op = '+';
    let mut stack: Vec<i32> = vec![];
    let mut current: i32 = 0;

    while let Some(ch) = chars.next() {
        if ch == '(' {
            current = calculate_expression(chars);
        } else if ch.is_ascii_digit() {
            current = current * 10 + ch.to_digit(10).unwrap_or_default() as i32;
        } else {
            if prev_op == '*' || prev_op == '/' {
                let prev = stack.pop().unwrap();
                stack.push(evaluate(prev, current, prev_op));
            } else {
                stack.push(evaluate(current, 0, prev_op));
            }

            if ch == ')' {
                break;
            }
            current = 0;
            prev_op = ch
        }
    }
    stack.iter().sum()
}

fn evaluate(left: i32, right: i32, op: char) -> i32 {
    eprintln!("evaluating {left} {op} {right}");
    match op {
        '+' => left,
        '-' => -left,
        '*' => left * right,
        _ => left / right,
    }
}
