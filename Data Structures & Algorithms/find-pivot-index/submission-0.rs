impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sums = vec![0; n];
        let mut prefix = 0;
        let mut postfix = 0;

        (0..n).for_each(|idx| {
            let rev_idx = n - idx - 1;
            prefix += nums[idx];
            postfix += nums[rev_idx];
            sums[idx] += prefix;
            sums[rev_idx] -= postfix;
        });
        for (idx, val) in sums.into_iter().enumerate() {
            if val == 0 {
                return idx as i32;
            }
        }
        -1
    }
}
