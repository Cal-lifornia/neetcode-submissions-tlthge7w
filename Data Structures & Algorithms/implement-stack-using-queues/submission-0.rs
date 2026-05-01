

pub struct MyStack {
    queue: Vec<i32>,
}

impl MyStack {
    pub fn new() -> Self {
        Self { queue: vec![] }
    }

    pub fn push(&mut self, x: i32) {
        self.queue.push(x);
    }

    pub fn pop(&mut self) -> i32 {
        self.queue.pop().unwrap()
    }

    pub fn top(&self) -> i32 {
        *self.queue.last().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}
