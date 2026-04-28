pub struct TicTacToe {
    rows: Vec<i32>,
    cols: Vec<i32>,
    diagonal: i32,
    anti_diagonal: i32,
    size: i32,
}

impl TicTacToe {
    pub fn new(n: i32) -> Self {
        let size = n as usize;
        Self {
            rows: vec![0; size],
            cols: vec![0; size],
            size: n,
            diagonal: 0,
            anti_diagonal: 0,
        }
    }

    pub fn make_move(&mut self, row: i32, col: i32, player: i32) -> i32 {
        let current_player = if player == 1 { 1 } else { -1 };
        let (r, c) = (row as usize, col as usize);
        self.rows[r] += current_player;
        self.cols[c] += current_player;

        if row == col {
            self.diagonal += current_player;
        }

        if col == (self.size - row - 1) {
            self.anti_diagonal += current_player;
        }
        if self.rows[r].abs() == self.size
            || self.cols[c].abs() == self.size
            || self.diagonal.abs() == self.size
            || self.anti_diagonal.abs() == self.size
        {
            player
        } else {
            0
        }
    }
}
