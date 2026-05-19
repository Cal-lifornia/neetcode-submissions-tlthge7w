use std::collections::BTreeMap;

impl Solution {
    pub fn brightest_position(lights: Vec<Vec<i32>>) -> i32 {
        let mut tree = BTreeMap::new();
        for light in lights {
            let position = light[0] as i64;
            let range = light[1] as i64;
            let start = position - range;
            let end = position + range;
            *tree.entry(start).or_insert(0) += 1;
            *tree.entry(end + 1).or_insert(0) -= 1;
        }

        let mut max_bright = 0;
        let mut curr = 0;
        let mut res = 0;

        for (pos, val) in &tree {
            curr += val;
            if curr > max_bright {
                max_bright = curr;
                res = *pos;
            }
        }
        res as i32
    }
}
