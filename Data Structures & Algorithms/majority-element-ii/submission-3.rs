impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut checked = HashSet::new();
        let mut res = vec![];

        for idx in 0..n {
            let num = nums[idx];
            if checked.contains(&num) {
                continue;
            } else {
                let count = nums.iter().filter(|&&val| val == num).count();
                if count > n / 3 {
                    res.push(num);
                }
                checked.insert(num);
            }
        }

        res
    }
}