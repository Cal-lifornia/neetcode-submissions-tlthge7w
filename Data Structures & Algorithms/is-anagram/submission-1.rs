impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let scorer = |val: String| -> HashMap<char, usize> {
            val.chars()
                .fold(HashMap::<char, usize>::new(), |mut map, ch| {
                    *map.entry(ch).or_default() += 1;
                    map
                })
        };
        scorer(s) == scorer(t)

    }
}
