impl Solution {
    pub fn decode_string(s: String) -> String {
        extract_pattern(1, &mut s.chars())
    }


}

fn extract_pattern(num: usize, chs: &mut std::str::Chars) -> String {
    let mut pattern = String::new();
    let mut next_num = String::new();
    while let Some(next) = chs.next()
        && next != ']'
    {
        if next.is_ascii_digit() {
            next_num.push(next);
        } else if next == '[' {
            let next_pattern = extract_pattern(next_num.parse().unwrap_or(0), chs);
            pattern.push_str(&next_pattern);
            next_num = String::new();
        } else {
            pattern.push(next);
        }
    }
    pattern.repeat(num)
}
