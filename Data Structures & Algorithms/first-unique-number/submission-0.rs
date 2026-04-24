pub struct FirstUnique {
    queue: Vec<i32>,
    map: HashMap<i32, usize>,
    first_unique: i32,
}

impl FirstUnique {
    pub fn new(nums: Vec<i32>) -> Self {
        let mut map = HashMap::<i32, usize>::new();
        let mut queue = vec![];
        nums.iter().for_each(|num| {
            if !map.contains_key(num) {
                queue.push(*num);
            }

            *map.entry(*num).or_default() += 1;
        });
        let mut out = Self {
            queue,
            map,
            first_unique: 0,
        };
        out.set_first_unique();
        out
    }

    fn set_first_unique(&mut self) {
        self.first_unique = self
            .queue
            .iter()
            .find(|num| {
                if let Some(amount) = self.map.get(num) {
                    *amount < 2
                } else {
                    false
                }
            })
            .cloned()
            .unwrap_or(-1);
    }

    pub fn show_first_unique(&self) -> i32 {
        self.first_unique
    }
    pub fn add(&mut self, value: i32) {
        if !self.map.contains_key(&value) {
            self.queue.push(value);
        }
        *self.map.entry(value).or_default() += 1;
        self.set_first_unique();
    }
}
