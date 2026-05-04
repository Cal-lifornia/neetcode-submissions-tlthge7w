impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut current_extracted = s;
        let count = k as usize;
        loop {
            let next_extracted = extract(&current_extracted, count);
            if next_extracted.len() == current_extracted.len() {
                return next_extracted;
            } else {
                current_extracted = next_extracted;
            }
        }
    }

}

fn extract(s: &str, count: usize) -> String {
    let mut extracted = String::new();
    let mut current_char = ' ';
    let mut word = String::new();
    for ch in s.chars() {
        if ch != current_char {
            if word.len() != count {
                extracted.push_str(&word);
            }
            word.clear();
            current_char = ch;
        }
        word.push(ch);
        if word.len() == count {
            word.clear();
        }
    }
    if word.len() != count {
        extracted.push_str(&word);
    }
    extracted
}

