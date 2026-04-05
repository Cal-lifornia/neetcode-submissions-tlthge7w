impl Solution {
    pub fn multiply(mat1: Vec<Vec<i32>>, mat2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        mat1.iter()
            .map(|arr| {
                let mut out = vec![0; mat2[0].len()];
                mat2.iter().enumerate().for_each(|(idx, arr2)| {
                    // eprintln!("idx: {idx}");
                    arr2.iter().enumerate().for_each(|(idy, val)| {
                        // eprintln!("out[{idy}] += {} * {val}", arr[idx]);
                        // eprintln!("{} += {}", out[idy], arr[idx] * val);
                        out[idy] += arr[idx] * val
                    });
                });
                out
            })
            .collect()

    }
}
