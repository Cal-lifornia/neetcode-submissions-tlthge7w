impl Solution {
    pub fn largest_unique_number(nums: Vec<i32>) -> i32 {
    let map = nums
        .iter()
        .fold(HashMap::<i32, usize>::new(), |mut map, val| {
            *map.entry(*val).or_default() += 1;
            map
        });

    map.iter().fold(
        -1,
        |largest, (key, val)| {
            if *val < 2 { largest.max(*key) } else { largest }
        },
    )

    }
}
