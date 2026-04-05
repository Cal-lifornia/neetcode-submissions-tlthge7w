impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut high_num: i32 = -1;
        let mut out = arr.clone();
        for num in out.iter_mut().rev() {
            if *num > high_num {
                // eprintln!("num > high_num; returning {high_num}");
                std::mem::swap(&mut high_num, &mut *num);
            } else {
                // eprintln!("num <= high_num; returning {high_num}");
                *num = high_num;
            }
        }
        out
    }
}
