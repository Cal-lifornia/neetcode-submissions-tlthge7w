pub struct HitCounter {
    hits: HashMap<i32, i32>,
    times: Vec<i32>,
}

impl HitCounter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            hits: HashMap::new(),
            times: vec![],
        }
    }

    pub fn hit(&mut self, timestamp: i32) {
        let entry = self.hits.entry(timestamp).or_default();
        if *entry == 0 {
            let point = self.times.partition_point(|&val| val <= timestamp);
            if point == self.times.len() {
                self.times.push(timestamp);
            } else {
                self.times.insert(point, timestamp);
            }
        }
        *entry += 1;
    }

    pub fn get_hits(&mut self, timestamp: i32) -> i32 {
        let start_time = (timestamp - 300).max(0);
        let first_idx = if start_time > 0 {
            self.times.partition_point(|&val| val <= start_time)
        } else {
            0
        };
        let last_idx = self.times.partition_point(|&val| val < timestamp);
        let mut out = 0;
        if last_idx == self.times.len() {
            for &time in &self.times[first_idx..last_idx] {
                out += *self.hits.get(&time).unwrap_or(&0);
            }
        } else {
            for &time in &self.times[first_idx..=last_idx] {
                out += *self.hits.get(&time).unwrap_or(&0);
            }
        }
        out
    }
}
