
pub struct RandomizedSet {
    map: HashMap<i32, usize>,
    nums: Vec<i32>,
}

impl RandomizedSet {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            nums: vec![],
        }
    }

    pub fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            true
        } else {
            let idx = self.nums.len();
            self.nums.push(val);
            self.map.insert(val, idx);
            false
        }
    }

    pub fn remove(&mut self, val: i32) -> bool {
        if let Some(&idx) = self.map.get(&val) {
            let last = *self.nums.last().unwrap();
            self.nums[idx] = last;
            self.map.insert(last, idx);
            self.nums.pop();
            self.map.remove(&val);
            true
        } else {
            false
        }

    }

    pub fn get_random(&self) -> i32 {
        self.nums[rand::thread_rng().gen_range(0..self.nums.len())]
    }
}

