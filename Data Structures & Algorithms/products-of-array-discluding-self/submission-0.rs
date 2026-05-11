impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut products = vec![1; n];
        let mut prefix = 1;
        (0..n).for_each(|idx| {
            products[idx] = prefix;
            prefix *= nums[idx];
        });

        let mut postfix = 1;

        (0..n).rev().for_each(|idx| {
            products[idx] *= postfix;
            postfix *= nums[idx];
        });
        products
    }
}
