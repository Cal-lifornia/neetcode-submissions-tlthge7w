impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
    let mut sub_iter = s.chars();
    let mut sub_ch = sub_iter.next();
    for ch in t.chars() {
        if let Some(sub_char) = sub_ch
            && sub_char == ch
        {
            sub_ch = sub_iter.next();
        } else if sub_ch.is_none() {
            return true;
        }
    }
    sub_ch.is_none()

    }
}
