impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::<i32, i32>::new();
        nums.into_iter().for_each(|num| {
            *map.entry(num).or_default() += 1;
        });
        let mut count = 0;
        for val in map.into_values() {
            if val == 1 {
                return -1;
            }
            count += (val as f64 / 3.0).ceil() as i32;
        }
        count
    }
}
