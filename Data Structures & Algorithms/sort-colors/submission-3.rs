impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut red = 0;
        let mut white = 0;
        let mut blue = 0;
        nums.iter().for_each(|colour| {
            if *colour == 0 {
                red += 1;
            } else if *colour == 1 {
                white += 1;
            } else {
                blue += 1;
            }
        });

        white += red;
        blue += white;

        (0..red).for_each(|idx| {
            nums[idx] = 0;
        });
        (red..white).for_each(|idx| {
            nums[idx] = 1;
        });
        (white..blue).for_each(|idx| {
            nums[idx] = 2;
        });
    }
}
