impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
    let mut high_num = arr.last().unwrap().to_owned();
    let mut out: Vec<i32> = arr
        .iter()
        .rev()
        .map(|num| {
            if *num > high_num {
                let old_high_num = high_num;
                high_num = *num;
                old_high_num
            } else {
                high_num
            }
        })
        .collect();
    out[0] = -1;
    out.reverse();
    out


    }
}
