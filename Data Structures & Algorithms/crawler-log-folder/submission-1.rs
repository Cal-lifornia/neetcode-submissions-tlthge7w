impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut distance = 0;

        logs.into_iter().for_each(|log| {
            if log.as_str() == "../" {
                if distance > 0 {
                    distance -= 1;
                }
            } else if log.as_str() != "./" {
                distance += 1;
            }
        });

        distance
    }

}
