pub struct WordDistance {
    words: HashMap<String, Vec<usize>>,
}

impl WordDistance {
    pub fn new(words_dict: Vec<String>) -> Self {
        let mut words = HashMap::<String, Vec<usize>>::new();
        words_dict.into_iter().enumerate().for_each(|(idx, word)| {
            words.entry(word).or_default().push(idx);
        });
        Self { words }
    }

    pub fn shortest(&mut self, word1: String, word2: String) -> i32 {
        let word1_vec = self.words.get(&word1).unwrap();
        let word2_vec = self.words.get(&word2).unwrap();
        let mut shortest = i32::MAX;
        for &idx in word1_vec {
            for &idy in word2_vec {
                let dist = idx.abs_diff(idy) as i32;
                if dist > 1 {
                    shortest = shortest.min(dist);
                } else {
                    return dist;
                }
            }
        }
        shortest
    }
}
