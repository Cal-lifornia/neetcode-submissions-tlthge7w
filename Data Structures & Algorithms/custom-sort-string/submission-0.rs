impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut custom_order = [usize::MAX; 26];
        order.bytes().enumerate().for_each(|(idx, ch)| {
            let place = (ch - b'a') as usize;
            custom_order[place] = idx + 1;
        });

        let mut s_bytes = s.bytes().collect::<Vec<u8>>();
        s_bytes.sort_by(|a, b| {
            let a_place = (a - b'a') as usize;
            let b_place = (b - b'a') as usize;
            custom_order[a_place].cmp(&custom_order[b_place])
        });
        String::from_utf8(s_bytes).unwrap()
    }
}
