impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        use std::collections::HashSet;
        let mut squaresets = vec![vec![HashSet::<char>::new(); 3]; 3];
        let mut columnsets = vec![HashSet::<char>::new(); 9];
        for y in 0..9 {
            let mut rowset = HashSet::<char>::new();
            for x in 0..9 {
                let val = board[y][x];
                if val != '.' {
                    let columnset = &mut columnsets[x];
                    let squareset = &mut squaresets[y / 3][x / 3];
                    if !squareset.insert(val) || !rowset.insert(val) || !columnset.insert(val) {
                        return false;
                    }
                }
            }
            rowset.clear();
        }
        true
    }
}
