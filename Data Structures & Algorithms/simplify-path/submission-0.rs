impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = vec![];
        for split in path.split('/') {
            if split == "." || split.is_empty() {
                continue;
            } else if split == ".." {
                stack.pop();
            } else {
                stack.push(format!("/{split}"));
            }
        }
        if stack.is_empty() {
            "/".into()
        } else {
            stack.join("")
        }
    }

}
