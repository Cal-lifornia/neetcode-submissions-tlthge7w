impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (idx, num) in nums.iter().enumerate() {
        for (idy, val) in nums.iter().enumerate().skip(idx + 1) {
            if val + num == target {
                return vec![idx as i32, idy as i32];
            }
        }
    }
    vec![]

    }
}
