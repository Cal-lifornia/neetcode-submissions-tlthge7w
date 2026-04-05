impl Solution {
    pub fn can_permute_palindrome(s: String) -> bool {
    let occurrences = s
        .chars()
        .fold(HashMap::<char, usize>::new(), |mut map, ch| {
            *map.entry(ch).or_default() += 1;
            map
        });

    let odds = occurrences
        .values()
        .filter(|val| *val % 2 != 0)
        .collect::<Vec<&usize>>();
    odds.len() <= 1

    }
}
