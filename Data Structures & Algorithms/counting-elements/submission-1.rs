impl Solution {
    pub fn count_elements(arr: Vec<i32>) -> i32 {
    let occurrences = arr
        .iter()
        .fold(HashMap::<i32, usize>::new(), |mut map, val| {
            *map.entry(*val).or_default() += 1;
            map
        });
    occurrences.iter().fold(0i32, |out, (key, val)| {
        if occurrences.contains_key(&(*key + 1)) {
            out + *val as i32
        } else {
            out
        }
    })

    }
}
