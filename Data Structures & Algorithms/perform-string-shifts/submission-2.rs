impl Solution {
    pub fn string_shift(s: String, shift: Vec<Vec<i32>>) -> String {
    let total_shift = shift.iter().fold(0i32, |out, val| {
        if val[0] == 0 {
            out - val[1]
        } else {
            out + val[1]
        }
    });
    let range = s.len();
    // eprintln!("total_shift: {total_shift}");
    let total_shift = if total_shift.is_negative() {
        if total_shift.abs() > range as i32 {
            // eprintln!(
            //     "|total_shift| % range = {}",
            //     total_shift.unsigned_abs() as usize % range
            // );
            range - (total_shift.unsigned_abs() as usize % range)
        } else {
            range + total_shift as usize
        }
    } else {
        total_shift as usize
    };
    // eprintln!("usize total_shift: {total_shift}");
    let mut out = vec!['a'; range];
    s.char_indices().for_each(|(idx, ch)| {
        let position = total_shift + idx;
        // eprintln!("position: {position}; range: {range}");
        if position >= range {
            out[position % range] = ch;
        } else {
            out[position] = ch;
        }
    });
    out.iter().collect()


    }
}
