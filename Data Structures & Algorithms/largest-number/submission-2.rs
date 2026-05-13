impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut arr = nums
            .into_iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>();
        arr.sort_by(|a, b| format!("{a}{b}").cmp(&format!("{b}{a}")).reverse());
        let largest = arr.join("");
        if largest.starts_with('0') {
            "0".into()
        } else {
            largest
        }
    }
}
