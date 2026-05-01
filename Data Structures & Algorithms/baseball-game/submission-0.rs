impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut record: Vec<i32> = vec![];
        operations.into_iter().for_each(|op| match op.as_str() {
            "+" => {
                record.push(record[record.len() - 2..].iter().sum());
            }
            "D" => {
                record.push(record.last().unwrap() * 2);
            }
            "C" => {
                record.pop();
            }
            num => {
                record.push(num.parse().unwrap());
            }
        });
        record.into_iter().sum()
    }
}
