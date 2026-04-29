impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details.iter().fold(0, |sum, value| {
            if value.get(11..13).unwrap_or("00").parse().unwrap_or(0) > 60 {
                sum + 1
            } else {
                sum
            }
        })
    }

}
