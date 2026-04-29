use std::collections::VecDeque;

pub struct ZigzagIterator {
    iter: Vec<i32>,
}

impl ZigzagIterator {
    fn new(v1: Vec<i32>, v2: Vec<i32>) -> Self {
        let mut v1_iter = v1.into_iter();
        let mut v2_iter = v2.into_iter();
        let mut iter = vec![];
        loop {
            let (left, right) = (v1_iter.next(), v2_iter.next());
            if matches!((left, right), (None, None)) {
                break;
            }
            if let Some(left) = left {
                iter.push(left);
            }
            if let Some(right) = right {
                iter.push(right);
            }
        }
        iter.reverse();
        Self { iter }
    }

    fn next(&mut self) -> i32 {
        self.iter.pop().unwrap_or(-1)
    }

    fn has_next(&self) -> bool {
        !self.iter.is_empty()
    }
}

