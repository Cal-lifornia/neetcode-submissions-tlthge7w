impl Solution {
pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = vec![];
    tokens.into_iter().for_each(|token| {
        match token.as_str() {
            "+" => {
                let res = stack.drain(stack.len() - 2..).sum();
                stack.push(res);
            }
            "-" => {
                let res = stack
                    .drain(stack.len() - 2..)
                    .reduce(|acc, e| acc - e)
                    .unwrap();
                stack.push(res);
            }
            "*" => {
                let res = stack
                    .drain(stack.len() - 2..)
                    .reduce(|acc, e| acc * e)
                    .unwrap();
                stack.push(res);
            }
            "/" => {
                let res = stack
                    .drain(stack.len() - 2..)
                    .reduce(|acc, e| acc / e)
                    .unwrap();
                stack.push(res);
            }
            _ => {
                stack.push(token.parse().unwrap());
            }
        }
    });
    stack.into_iter().sum()
}

}
