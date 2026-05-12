impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut current_sum = 0;
        let mut prefixes = HashMap::new();
        prefixes.insert(0, 1);
        for &num in &nums {
            current_sum += num;
            let diff = current_sum - k;
            count += prefixes.get(&diff).unwrap_or(&0);
            *prefixes.entry(current_sum).or_insert(0) += 1;
        }
        count
    }
}
