impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut encoded = String::new();
        strs.into_iter().for_each(|item| {
            let len = item.len();
            encoded.push_str(format!("{len}\n{item}").as_str());
        });
        encoded
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut decoded = vec![];
        let s_bytes = s.into_bytes();
        let mut current = 0;
        let mut temp_len = String::new();
        while current < s_bytes.len() {
            if s_bytes[current] != b'\n' {
                temp_len.push(s_bytes[current] as char);
                current += 1;
            } else {
                current += 1;
                let len = temp_len.parse::<usize>().unwrap();
                temp_len.clear();
                let word = str::from_utf8(&s_bytes[current..(current + len)])
                    .unwrap()
                    .to_string();
                decoded.push(word);
                current += len;
            }
        }

        decoded
    }

}
