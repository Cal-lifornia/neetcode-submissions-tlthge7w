impl Solution {
    pub fn confusing_number(n: i32) -> bool {
    let flipped = |val: char| -> Option<char> {
        match val {
            '0' => Some('0'),
            '1' => Some('1'),
            '6' => Some('9'),
            '8' => Some('8'),
            '9' => Some('6'),
            _ => None,
        }
    };

    let mut flipped_str = String::new();

    for ch in n.to_string().chars().rev() {
        if let Some(val) = flipped(ch) {
            flipped_str.push(val);
        } else {
            return false;
        }
    }
    if let Ok(flipped) = flipped_str.parse::<i32>()
        && flipped != n
    {
        true
    } else {
        false
    }

    }
}
