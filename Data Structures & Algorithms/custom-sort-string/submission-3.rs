impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut custom_order = [26usize; 26];
        order.bytes().enumerate().for_each(|(idx, ch)| {
            let place = (ch - b'a') as usize;
            custom_order[place] = idx + 1;
        });

        let mut s_bytes = s.into_bytes();
        s_bytes.sort_by_key(|ch| custom_order[(ch - b'a') as usize]);
        String::from_utf8(s_bytes).unwrap()
    }
}
