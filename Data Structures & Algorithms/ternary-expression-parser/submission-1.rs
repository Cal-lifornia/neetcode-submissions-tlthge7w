impl Solution {
    pub fn parse_ternary(expression: String) -> String {
        let mut chars = expression.chars();
        parse_expression(&mut chars)
    }
}
fn parse_expression(chars: &mut std::str::Chars) -> String {
    if let Some(current) = chars.next() {
        if chars.next() == Some('?') {
            let left = parse_expression(chars);
            let right = parse_expression(chars);
            if current == 'T' { left } else { right }
        } else {
            current.into()
        }
    } else {
        "".into()
    }
}