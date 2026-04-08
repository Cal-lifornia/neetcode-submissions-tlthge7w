pub struct CandyCrushGrid {
    grid: Vec<Vec<i32>>,
    x_len: usize,
    y_len: usize,
}

impl CandyCrushGrid {
    pub fn new(grid: Vec<Vec<i32>>) -> Self {
        let x_len = grid.len();
        let y_len = grid[0].len();
        Self { grid, x_len, y_len }
    }

    fn get_remove_list(&mut self) -> Option<Vec<HashSet<usize>>> {
        let mut remove_list = vec![HashSet::<usize>::new(); self.x_len];
        (0..self.x_len).for_each(|idx| {
            (0..self.y_len).for_each(|idy| {
                let current = self.grid[idx][idy];
                if current == 0 {
                    return;
                }
                if idx > 0 && idx < self.x_len - 1 {
                    let prev_x = self.grid[idx - 1][idy];
                    let next_x = self.grid[idx + 1][idy];

                    if current == prev_x && current == next_x {
                        remove_list[idx - 1].insert(idy);
                        remove_list[idx].insert(idy);
                        remove_list[idx + 1].insert(idy);
                    }
                }

                if idy > 0 && idy < self.y_len - 1 {
                    let prev_y = self.grid[idx][idy - 1];
                    let next_y = self.grid[idx][idy + 1];
                    if current == prev_y && current == next_y {
                        remove_list[idx].insert(idy - 1);
                        remove_list[idx].insert(idy);
                        remove_list[idx].insert(idy + 1);
                    }
                }
            });
        });
        if remove_list.iter().all(|set| set.is_empty()) {
            None
        } else {
            Some(remove_list)
        }
    }
}
impl Solution {
    pub fn candy_crush(board: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let grid = rotate_grid(board);
        let mut cc_grid = CandyCrushGrid::new(grid);
        while let Some(remove_list) = cc_grid.get_remove_list() {
            (0..cc_grid.x_len).for_each(|idx| {
                (0..cc_grid.y_len).rev().for_each(|idy| {
                    if remove_list[idx].contains(&idy) {
                        cc_grid.grid[idx].remove(idy);
                    }
                });
                cc_grid.grid[idx].resize(cc_grid.y_len, 0);
            });
        }
        rerotate_grid(cc_grid.grid)
    }

}

fn rotate_grid(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let row_len = grid.first().map_or(0, |row| row.len());
    let column_len = grid.len();
    let mut out = vec![vec![0i32; column_len]; row_len];
    (0..row_len).for_each(|row| {
        (0..column_len).for_each(|col| out[row][column_len - 1 - col] = grid[col][row]);
    });
    out
}
fn rerotate_grid(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let row_len = grid.first().map_or(0, |row| row.len());
    let column_len = grid.len();
    let mut out = vec![vec![0i32; column_len]; row_len];
    (0..row_len).for_each(|row| {
        (0..column_len).for_each(|col| out[row_len - 1 - row][col] = grid[col][row]);
    });
    out
}
