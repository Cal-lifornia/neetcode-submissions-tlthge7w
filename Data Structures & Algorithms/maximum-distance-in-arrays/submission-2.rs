impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut min_val = *arrays[0].first().unwrap();
        let mut max_val = *arrays[0].last().unwrap();
        arrays
            .iter()
            .zip(arrays.iter().skip(1))
            .fold(0, |dist, (arr1, arr2)| {
                min_val = min_val.min(*arr1.first().unwrap());
                max_val = max_val.max(*arr1.last().unwrap());
                dist.max(
                    (max_val - arr2.first().unwrap())
                        .abs()
                        .max(arr2.last().unwrap() - min_val)
                        .abs(),
                )
            })
    }
}
