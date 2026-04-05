impl Solution {
    pub fn calculate_time(keyboard: String, word: String) -> i32 {
    let map = keyboard
        .char_indices()
        .fold(HashMap::<char, i32>::new(), |mut map, (idx, ch)| {
            map.insert(ch, idx as i32);
            map
        });
    let first = {
        let Some(first) = word.chars().next() else {
            return 0;
        };
        *map.get(&first).unwrap_or(&0)
    };
    word.chars()
        .zip(word.chars().skip(1))
        .fold(first, |out, (ch1, ch2)| {
            if let Some(val1) = map.get(&ch1)
                && let Some(val2) = map.get(&ch2)
            {
                eprintln!("{out} + |{val1} - {val2}|");
                out + (val1 - val2).abs()
            } else {
                out
            }
        })

    }
}
