impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut freqmap = std::collections::HashMap::<i32, usize>::new();
        for &num in &nums {
            *freqmap.entry(num).or_default() += 1;
        }

        let mut nums = nums;
        nums.sort_by(|num1, num2| {
            freqmap
                .get(num1)
                .unwrap()
                .cmp(freqmap.get(num2).unwrap())
                .then(num1.cmp(num2).reverse())
        });
        nums
    }
}
