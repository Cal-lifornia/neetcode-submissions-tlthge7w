use std::collections::{BTreeMap, HashMap};

use itertools::Itertools;

#[derive(Debug)]
pub struct Leaderboard {
    players: HashMap<i32, i32>,
    sorted_scores: BTreeMap<i32, i32>,
}

impl Leaderboard {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            sorted_scores: BTreeMap::new(),
        }
    }

    pub fn add_score(&mut self, player_id: i32, score: i32) {
        let entry_score = self.players.entry(player_id).or_default();
        let old_score = *entry_score;
        *entry_score += score;
        if old_score > 0 {
            *self.sorted_scores.entry(old_score).or_insert(1) -= 1;
        }
        *self.sorted_scores.entry(*entry_score).or_default() += 1;
    }

    pub fn top(&self, k: i32) -> i32 {
        let mut n = k;
        self.sorted_scores
            .iter()
            .rev()
            .fold_while(0, |sum, (score, count)| {
                if n > 0 {
                    let c = (*count).min(n);
                    n -= c;
                    itertools::FoldWhile::Continue(sum + (score * c))
                } else {
                    itertools::FoldWhile::Done(sum)
                }
            })
            .into_inner()
    }

    pub fn reset(&mut self, player_id: i32) {
        let score = self.players.entry(player_id).or_default();
        if *score > 0 {
            *self.sorted_scores.entry(*score).or_insert(1) -= 1;
        }
        *score = 0;
    }
}
