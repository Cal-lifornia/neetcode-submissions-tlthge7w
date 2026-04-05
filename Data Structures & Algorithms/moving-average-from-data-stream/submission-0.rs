struct MovingAverage {
    size: usize,
    queue: std::collections::VecDeque<i32>,
}

impl MovingAverage {
    pub fn new(size: i32) -> Self {
        Self {
            size: size as usize,
            queue: std::collections::VecDeque::with_capacity(size as usize),
        }
    }

    pub fn next(&mut self, val: i32) -> f64 {
        if self.queue.len() == self.size {
            self.queue.pop_front();
        }
        self.queue.push_back(val);
        self.queue.iter().sum::<i32>() as f64 / self.queue.len() as f64
    }
}

