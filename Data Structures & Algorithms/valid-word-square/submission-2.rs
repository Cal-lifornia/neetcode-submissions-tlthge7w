impl Solution {
    pub fn valid_word_square(words: Vec<String>) -> bool {
    let words: Vec<Vec<u8>> = words.iter().map(|val| val.as_bytes().to_vec()).collect();

    for (y, word) in words.iter().enumerate() {
        for (x, ch) in word.iter().enumerate().rev() {
            // eprintln!("x: {x}; y: {y}");
            if let Some(val) = words.get(x).unwrap_or(&vec![]).get(y)
                && val == ch
            {
                // eprintln!("val: {val}; ch: {y}");
                continue;
            } else {
                return false;
            }
        }
    }

    true


    }
}
