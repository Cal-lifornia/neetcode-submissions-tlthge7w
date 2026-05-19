pub struct SparseVector {
    nums: HashMap<usize, i32>,
}

impl SparseVector {
    pub fn new(nums: Vec<i32>) -> Self {
        let mut map = HashMap::<usize, i32>::new();
        for (idx, num) in nums.into_iter().enumerate() {
            if num > 0 {
                map.insert(idx, num);
            }
        }
        Self { nums: map }
    }

    pub fn dot_product(&self, vec: &SparseVector) -> i32 {
        let mut product = 0;
        for (key, &val) in &self.nums {
            product += val * *vec.nums.get(key).unwrap_or(&0);
        }
        product
    }
}

