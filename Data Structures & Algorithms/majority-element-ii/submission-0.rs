impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() / 3;
        let mut count = std::collections::HashMap::<i32, usize>::new();
        for num in nums {
            *count.entry(num).or_default() += 1;
        }

        count
            .into_iter()
            .filter_map(|(key, val)| if val > n { Some(key) } else { None })
            .collect()
    }
}
