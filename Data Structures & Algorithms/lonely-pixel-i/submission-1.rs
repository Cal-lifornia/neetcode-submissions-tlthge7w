impl Solution {
pub fn find_lonely_pixel(picture: Vec<Vec<char>>) -> i32 {
    let m = picture.len();
    let n = picture[0].len();
    let mut pixels = vec![];
    (0..m).for_each(|y| {
        let mut row_entries = vec![];
        (0..n).for_each(|x| {
            if picture[y][x] == 'B' {
                row_entries.push(x);
            }
        });
        // eprintln!("row_entries: {row_entries:#?}");
        if row_entries.len() == 1 {
            pixels.push(*row_entries.first().unwrap());
        }
    });

    // eprintln!("pixels: {pixels:#?}");
    let mut out = 0;
    pixels.iter().for_each(|x| {
        let mut count = 0;
        (0..m).for_each(|col| {
            if picture[col][*x] == 'B' {
                count += 1;
            }
        });
        if count == 1 {
            out += 1;
        }
    });
    out
}

}
