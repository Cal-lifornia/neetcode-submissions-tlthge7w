impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut min_val = *arrays[0].first().unwrap();
        let mut max_val = *arrays[0].last().unwrap();
        arrays.iter().skip(1).fold(0, |mut dist, arr| {
            dist = dist.max(
                (max_val - arr.first().unwrap())
                    .abs()
                    .max(arr.last().unwrap() - min_val)
                    .abs(),
            );
            min_val = min_val.min(*arr.first().unwrap());
            max_val = max_val.max(*arr.last().unwrap());
            dist
        })
    }

}
