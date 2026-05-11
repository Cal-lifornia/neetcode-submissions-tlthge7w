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
    let mut idx = 0;
    while idx < s.len() {
        let delimiter = s[idx..].find('\n').unwrap() + idx;
        let len = s[idx..delimiter].parse::<usize>().unwrap();
        idx = delimiter + 1;
        decoded.push(s[idx..(idx + len)].to_string());
        idx += len;
    }

    decoded
}


}
