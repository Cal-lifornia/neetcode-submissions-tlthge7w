impl Solution {
pub fn most_visited_pattern(
    username: Vec<String>,
    timestamp: Vec<i32>,
    website: Vec<String>,
) -> Vec<String> {
    let mut usermap = HashMap::<&String, Vec<(&i32, &String)>>::new();
    for idx in 0..username.len() {
        usermap
            .entry(&username[idx])
            .or_default()
            .push((&timestamp[idx], &website[idx]));
    }
    let mut patterns = HashMap::<Vec<String>, usize>::new();
    usermap.values_mut().for_each(|arr| {
        arr.sort_by(|a, b| a.0.cmp(b.0));
    });
    usermap.into_values().for_each(|arr| {
        arr.windows(3).for_each(|patt| {
            *patterns
                .entry(vec![
                    patt[0].1.clone(),
                    patt[1].1.clone(),
                    patt[2].1.clone(),
                ])
                .or_default() += 1;
        });
    });
    patterns
        .into_iter()
        .max_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0).reverse()))
        .unwrap()
        .0
}
}
