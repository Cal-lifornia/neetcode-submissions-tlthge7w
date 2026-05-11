impl Solution {
pub fn most_visited_pattern(
    username: Vec<String>,
    timestamp: Vec<i32>,
    website: Vec<String>,
) -> Vec<String> {
    let n = username.len();
    let mut usermap = HashMap::<&str, Vec<&str>>::new();
    let mut times: Vec<(i32, usize)> = (0..n).map(|idx| (timestamp[idx], idx)).collect();
    times.sort();
    for &(_, idx) in &times {
        usermap
            .entry(&username[idx])
            .or_default()
            .push(&website[idx]);
    }
    let mut patterns = HashMap::<String, usize>::new();
    usermap.into_values().for_each(|arr| {
        arr.windows(3).for_each(|patt| {
            *patterns
                .entry(format!("{}#{}#{}", patt[0], patt[1], patt[2]))
                .or_default() += 1;
        });
    });
    patterns
        .into_iter()
        .max_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0).reverse()))
        .unwrap()
        .0
        .split('#')
        .map(|val| val.to_string())
        .collect()
}
}
