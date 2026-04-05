use std::collections::HashSet;

impl Solution {
    pub fn are_sentences_similar(
        sentence1: Vec<String>,
        sentence2: Vec<String>,
        similar_pairs: Vec<Vec<String>>,
    ) -> bool {
    if sentence1.len() != sentence2.len() {
        return false;
    }
    let mut similar_map: HashMap<&String, HashSet<&String>> = HashMap::new();
    similar_pairs.iter().for_each(|val| {
        similar_map.entry(&val[0]).or_default().insert(&val[1]);
        similar_map.entry(&val[1]).or_default().insert(&val[0]);
    });
    for (idx, word1) in sentence1.iter().enumerate() {
        let word2 = &sentence2[idx];
        if word1 == word2 {
            continue;
        } else if let Some(set1) = similar_map.get(word1)
            && set1.contains(word2)
        {
            continue;
        } else {
            return false;
        }
    }
    true

    }
}
