impl Solution {
    pub fn shortest_way(source: String, target: String) -> i32 {
        let mut occurrences: Vec<Vec<usize>> = vec![vec![]; 26];
        source.bytes().enumerate().for_each(|(idx, val)| {
            let place = val - b'a';
            occurrences[place as usize].push(idx);
        });
        // eprintln!("occurrences: {occurrences:#?}");

        let mut output = 0;
        let target_bytes = target.bytes().collect::<Vec<_>>();
        let mut current = 0;
        let len = target.len();
        while current < len {
            let place = (target_bytes[current] - b'a') as usize;
            // eprintln!("current: {current}; place: {place}");
            let occurrence = &occurrences[place];
            let Some(max) = occurrence
                .iter()
                .map(|idx| {
                    let mut temp_current = current + 1;
                    let mut relative_idx = *idx;
                    while temp_current < len {
                        let next_place = (target_bytes[temp_current] - b'a') as usize;
                        // eprintln!("temp_current: {temp_current}; next_place: {next_place}");
                        let next_occurrence = &occurrences[next_place];
                        if let Some(next) = next_occurrence.iter().find(|val| **val > relative_idx) {
                            relative_idx = *next;
                            temp_current += 1;
                        } else {
                            break;
                        }
                    }
                    temp_current
                })
                .max()
            else {
                return -1;
            };
            output += 1;
            current = max;
        }

        if output > 0 { output } else { -1 }
    }


}
