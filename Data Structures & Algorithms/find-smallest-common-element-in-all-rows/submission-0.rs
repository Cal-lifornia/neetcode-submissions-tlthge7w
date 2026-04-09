impl Solution {
    pub fn smallest_common_element(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat[0].len();
        for idx in 0..n {
            let val = mat[0][idx];
            if mat
                .iter()
                .skip(1)
                .all(|arr| arr.binary_search(&val).is_ok())
            {
                return val;
            }
        }
        -1
    }

}
