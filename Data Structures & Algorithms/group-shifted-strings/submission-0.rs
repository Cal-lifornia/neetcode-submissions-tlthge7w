impl Solution {
    pub fn group_strings(strings: Vec<String>) -> Vec<Vec<String>> {
        let map = strings
            .iter()
            .fold(HashMap::<String, Vec<String>>::new(), |mut map, val| {
                let val_bytes = val.as_bytes();
                // if val.len() > 1 {
                let diff: String = val_bytes.iter().zip(val_bytes.iter().skip(1)).fold(
                    String::new(),
                    |mut diff: String, (byte1, byte2)| {
                        let ch = ((*byte2 as i32 - *byte1 as i32 + 26) % 26) as u8 + b'a';
                        diff.push(ch as char);
                        diff
                    },
                );
                map.entry(diff).or_default().push(val.clone());
                map
                // } else {
                //     let diff = (val_bytes[0] + 26) % 26 + b'a';
                //     map.entry((diff as char).to_string())
                //         .or_default()
                //         .push(val.clone());
                //     map
                // }
            });
        map.values().cloned().collect()
    }

}
