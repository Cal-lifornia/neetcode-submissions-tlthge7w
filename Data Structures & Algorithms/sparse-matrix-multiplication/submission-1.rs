impl Solution {
    pub fn multiply(mat1: Vec<Vec<i32>>, mat2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        mat1.iter()
            .map(|arr| {
                let mut out = vec![];
                (0..mat2[0].len()).for_each(|idx| {
                    let mut val = 0;
                    (0..arr.len()).for_each(|idy| {
                        val += arr[idy] * mat2[idy][idx];
                    });
                    out.push(val);
                });
                out
            })
            .collect()


    }
}
