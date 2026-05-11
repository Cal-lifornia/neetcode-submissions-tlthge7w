pub struct NumMatrix {
    matrix: Vec<Vec<i32>>,
}

impl NumMatrix {
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        Self { matrix }
    }

    pub fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let (r1, c1, r2, c2) = (row1 as usize, col1 as usize, row2 as usize, col2 as usize);
        let mut sum = 0;
        for row in r1..=r2 {
            // eprintln!("row: {row}");
            // eprintln!("{:?}", &self.matrix[row][c1..=c2]);
            sum += self.matrix[row][c1..=c2].iter().sum::<i32>();
        }
        sum
    }
}
