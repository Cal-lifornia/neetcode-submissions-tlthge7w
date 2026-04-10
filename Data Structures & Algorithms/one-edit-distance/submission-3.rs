impl Solution {
    pub fn is_one_edit_distance(s: String, t: String) -> bool {
        if s == t {
            return false;
        }
        let difference = (s.len() as i32 - t.len() as i32).abs();
        match difference {
            0 => {
                let mut matched = false;
                for (ch1, ch2) in s.chars().zip(t.chars()) {
                    if ch1 != ch2 {
                        if matched {
                            return false;
                        } else {
                            matched = true;
                        }
                    }
                }
                matched
            }
            1 => {
                if s.len() > t.len() {
                    let mut s_clone = s.clone();
                    s_clone.pop();
                    if t == s_clone {
                        return true;
                    }
                } else {
                    let mut t_clone = t.clone();
                    t_clone.pop();

                    if s == t_clone {
                        return true;
                    }
                }
                for ((ids, chs), cht) in s.char_indices().zip(t.chars()) {
                    if chs != cht {
                        if s.len() > t.len() {
                            let mut s_clone = s.clone();
                            s_clone.remove(ids);
                            return t == s_clone;
                        } else {
                            let mut t_clone = t.clone();
                            t_clone.remove(ids);
                            return s == t_clone;
                        }
                    }
                }
                false
            }
            _ => false,
        }
    }


}
