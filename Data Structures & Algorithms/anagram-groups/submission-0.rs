impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    if strs.is_empty() {
        return vec![];
    }
    let map = strs.iter().enumerate().fold(
        HashMap::new(),
        |mut map: HashMap<Vec<char>, Vec<usize>>, (idx, val)| {
            let mut chrs = val.chars().collect::<Vec<char>>();
            chrs.sort_unstable();
            let entry = map.entry(chrs).or_default();
            entry.push(idx);
            map
        },
    );
    map.values()
        .map(|val| {
            val.iter()
                .map(|idx| strs[*idx].clone())
                .collect::<Vec<String>>()
        })
        .collect()

    }
}
