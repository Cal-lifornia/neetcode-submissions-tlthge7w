impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut chs = s.chars();
        validate(&mut chs, false)
    }
}

fn validate(chs: &mut std::str::Chars, open: bool) -> String {
    let mut out = String::new();
    while let Some(ch) = chs.next() {
        if ch == ')' {
            if open {
                return format!("({out})");
            }
        } else if ch == '(' {
            out.push_str(&validate(chs, true));
        } else {
            out.push(ch);
        }
    }
    out
}