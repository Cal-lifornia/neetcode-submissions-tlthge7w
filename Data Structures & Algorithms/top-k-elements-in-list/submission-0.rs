impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = nums
        .iter()
        .fold(HashMap::<i32, i32>::new(), |mut map, val| {
            *map.entry(*val).or_default() += 1;
            map
        });
    let mut out: Vec<i32> = vec![];
    for _ in 0..k {
        let (key, _) = &map.iter().max_by_key(|(_, val)| **val).unwrap();
        let key = **key;
        map.remove(&key);
        out.push(key);
    }
    out

    }
}
