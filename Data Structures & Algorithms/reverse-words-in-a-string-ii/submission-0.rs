impl Solution {
    pub fn reverse_words(s: &mut Vec<char>) {
        s.reverse();
        let mut start = 0;
        while start < s.len() {
            let mut end = start;
            while end < s.len() && s[end] != ' ' {
                end += 1;
            }
            s[start..end].reverse();
            start = end + 1;
        }
    }

}
