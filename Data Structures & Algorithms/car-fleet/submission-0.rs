impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let n = position.len();
        let mut pos_speed: Vec<(i32, i32)> = vec![];
        (0..n).for_each(|idx| {
            pos_speed.push((position[idx], speed[idx]));
        });
        pos_speed.sort_by(|val1, val2| val1.0.cmp(&val2.0).reverse());

        let mut fleets = 1;
        let mut prev_time = (target - pos_speed[0].0) as f64 / pos_speed[0].1 as f64;
        for (p, s) in pos_speed.into_iter().skip(1) {
            let time = (target - p) as f64 / s as f64;
            if time > prev_time {
                fleets += 1;
                prev_time = time;
            }
        }
        fleets
    }
}
