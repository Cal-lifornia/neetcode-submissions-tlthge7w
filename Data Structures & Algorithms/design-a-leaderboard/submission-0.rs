use std::collections::HashMap;

pub struct Leaderboard {
    board: HashMap<i32, i32>,
}

impl Leaderboard {
    pub fn new() -> Self {
        Self {
            board: HashMap::new(),
        }
    }

    pub fn add_score(&mut self, player_id: i32, score: i32) {
        *self.board.entry(player_id).or_default() += score;
    }

    pub fn top(&self, k: i32) -> i32 {
        let mut vals = self.board.values().copied().collect::<Vec<i32>>();
        vals.sort();
        vals[vals.len() - k as usize..].iter().sum()
    }

    pub fn reset(&mut self, player_id: i32) {
        *self.board.entry(player_id).or_default() = 0;
    }
}
