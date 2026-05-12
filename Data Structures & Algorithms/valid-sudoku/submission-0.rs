impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        use std::collections::HashSet;
        let mut squaresets = vec![vec![HashSet::<char>::new(); 3]; 3];
        for y in 0..9 {
            let mut rowset = HashSet::<char>::new();
            let mut columnset = HashSet::<char>::new();
            for x in 0..9 {
                let squareset = &mut squaresets[y / 3][x / 3];
                if board[y][x] != '.' && !squareset.insert(board[y][x]) {
                    return false;
                }
                if board[y][x] != '.' && !rowset.insert(board[y][x]) {
                    return false;
                }
                if board[x][y] != '.' && !columnset.insert(board[x][y]) {
                    return false;
                }
            }
            rowset.clear();
            columnset.clear();
        }
        true
    }
}
