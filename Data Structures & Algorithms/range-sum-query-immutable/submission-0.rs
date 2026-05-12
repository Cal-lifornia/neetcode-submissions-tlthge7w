pub struct NumArray {
    array_sum: Vec<i32>,
}

impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut array_sum = vec![0; n];
        let mut sum = 0;
        (0..n).for_each(|idx| {
            sum += nums[idx];
            array_sum[idx] = sum;
        });

        Self {
            // array: nums,
            array_sum,
        }
    }

    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        if left > 0 {
            self.array_sum[right as usize] - self.array_sum[(left - 1) as usize]
        } else {
            self.array_sum[right as usize]
        }
    }
}
