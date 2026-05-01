impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let count = temperatures.len();
        let mut results: Vec<i32> = vec![0i32; count];
        (0..temperatures.len()).for_each(|idx| {
            let mut result = 0;
            for temp in temperatures[idx + 1..].iter() {
                result += 1;
                if temp > &temperatures[idx] {
                    results[idx] = result;
                    break;
                }
            }
        });
        results
    }
}
