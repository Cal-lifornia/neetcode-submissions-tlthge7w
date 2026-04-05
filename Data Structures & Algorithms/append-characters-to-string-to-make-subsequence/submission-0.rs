impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
    let mut sub_iter = t.chars();
    let mut sub_ch = sub_iter.next();
    for ch in s.chars() {
        if let Some(sub_char) = sub_ch
            && sub_char == ch
        {
            sub_ch = sub_iter.next();
        } else if sub_ch.is_none() {
            return 0;
        }
    }
    if sub_ch.is_some() {
        (sub_iter.count() + 1) as i32
    } else {
        0
    }

    }
}
