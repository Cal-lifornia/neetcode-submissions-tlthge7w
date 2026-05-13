impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let total = nums.iter().sum::<i32>();
        if nums.len() == 1 {
            return false;
        }
        let mut prefix_sum = 0;
        for &num in &nums {
            let diff_modulo = (total - prefix_sum) % k;
            prefix_sum += num;
            let prefix_modulo = prefix_sum % k;
            if diff_modulo == 0 || prefix_modulo == 0 {
                return true;
            }
        }
        false
    }
}
