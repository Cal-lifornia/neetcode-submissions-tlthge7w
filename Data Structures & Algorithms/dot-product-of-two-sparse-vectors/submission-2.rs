struct SparseVector {
    pairs: Vec<(usize, i32)>,
}

impl SparseVector {
    fn new(nums: Vec<i32>) -> Self {
        let mut pairs = Vec::new();
        for (i, &num) in nums.iter().enumerate() {
            if num != 0 {
                pairs.push((i, num));
            }
        }
        SparseVector { pairs }
    }

    // Return the dot_product of two sparse vectors
    fn dot_product(&self, vec: &SparseVector) -> i32 {
        let mut result = 0;
        let mut p = 0;
        let mut q = 0;

        while p < self.pairs.len() && q < vec.pairs.len() {
            if self.pairs[p].0 == vec.pairs[q].0 {
                result += self.pairs[p].1 * vec.pairs[q].1;
                p += 1;
                q += 1;
            } else if self.pairs[p].0 > vec.pairs[q].0 {
                q += 1;
            } else {
                p += 1;
            }
        }

        result
    }
}
