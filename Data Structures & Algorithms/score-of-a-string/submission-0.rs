impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        s.chars()
            .zip(s.chars().skip(1))
            .fold(0i32, |acc, (ch1, ch2)| {
                acc + (u8::try_from(ch2).unwrap() as i32 - u8::try_from(ch1).unwrap() as i32).abs()
            })
    }
}
