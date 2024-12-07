pub struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }
        let mut grid = grid;
        let rows = grid.len();
        let cols = grid[0].len();
        let mut result: i32 = 0;
        fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize, rows: usize, cols: usize) {
            if i >= rows || j >= cols || grid[i][j] == '0' {
                return;
            }
            dfs(grid, i - 1, j, rows, cols);
            dfs(grid, i + 1, j, rows, cols);
            dfs(grid, i, j - 1, rows, cols);
            dfs(grid, i, j + 1, rows, cols);
        }

        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == '1' {
                    result += 1;
                    dfs(&mut grid, i, j, rows, cols);
                }
            }
        }
        result
    }
}
