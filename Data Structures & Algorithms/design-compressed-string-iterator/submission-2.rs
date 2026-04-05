pub struct StringIterator {
    arr: Vec<(char, usize)>,
    current_char: char,
    count: usize,
    ended: bool,
}

impl StringIterator {
    pub fn new(compressed_string: String) -> Self {
        let mut count_str = String::new();
        let mut current_char = ' ';
        let mut arr: Vec<(char, usize)> = vec![];
        for ch in compressed_string.chars() {
            if ch.is_numeric() {
                count_str.push(ch);
            } else {
                if !count_str.is_empty() {
                    let count = std::mem::take(&mut count_str).parse::<usize>().unwrap();
                    arr.push((current_char, count));
                }
                current_char = ch;
            }
        }
        let count = std::mem::take(&mut count_str).parse::<usize>().unwrap();
        arr.push((current_char, count));
        arr.reverse();
        Self {
            arr,
            current_char: ' ',
            count: 0,
            ended: false,
        }
    }
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> char {
        if self.ended {
            ' '
        } else if self.count > 0 {
            self.count -= 1;
            self.current_char
        } else if let Some((ch, count)) = self.arr.pop() {
            self.count = count - 1;
            self.current_char = ch;
            self.current_char
        } else {
            self.ended = true;
            ' '
        }
    }

    pub fn has_next(&self) -> bool {
        self.count > 0 || !self.arr.is_empty() || !self.ended
    }
}
