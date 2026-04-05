impl Solution {
    pub fn anagram_mappings(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();
    let mut results = vec![];
    nums1.iter().for_each(|num1| {
        if let Some(idx) = map.get(num1) {
            results.push(*idx as i32);
        } else {
            for (idx, num2) in nums2.iter().enumerate() {
                if num1 == num2 {
                    map.insert(*num1, idx);
                    results.push(idx as i32);
                    break;
                }
            }
        }
    });
    results

    }
}
