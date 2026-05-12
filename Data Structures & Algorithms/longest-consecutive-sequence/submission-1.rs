impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut nums = nums;
        nums.sort();
        let mut longest = 0;
        let mut streak = 1;
        let mut prev_num = nums[0];
        for num in nums.into_iter().skip(1) {
            if num == prev_num {
                continue;
            }
            if num == prev_num + 1 {
                streak += 1;
            } else {
                longest = longest.max(streak);
                streak = 1;
            }
            prev_num = num;
        }
        longest.max(streak)
    }
}
