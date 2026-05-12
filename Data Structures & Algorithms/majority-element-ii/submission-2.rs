impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = HashSet::new();
        for &num in &nums {
            let count = nums.iter().filter(|&&x| x == num).count();
            if count > n / 3 {
                res.insert(num);
            }
        }
        res.into_iter().collect()
    }
}