impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        let mut less = true;
        for idx in 0..nums.len() - 1 {
            let val = nums[idx];
            if less {
                if val > nums[idx + 1] {
                    nums.swap(idx, idx + 1);
                }
                less = false;
            } else {
                if val < nums[idx + 1] {
                    nums.swap(idx, idx + 1);
                }
                less = true;
            }
        }
    }
}
