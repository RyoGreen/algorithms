pub struct Solution;

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut grid = grid;
        let mut max_area = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    let area = Self::dfs(i, j, &mut grid);
                    max_area = max_area.max(area + 1);
                }
            }
        }
        return max_area;
    }
    fn dfs(i: usize, j: usize, grid: &mut Vec<Vec<i32>>) -> i32 {
        if i >= grid.len() || j >= grid[0].len() || grid[i][j] == 0 {
            return 0;
        }
        let mut max_area = 0;
        if grid[i][j] == 1 {
            grid[i][j] = 0;
            max_area = Self::dfs(i.wrapping_sub(1), j, grid)
                + Self::dfs(i + 1, j, grid)
                + Self::dfs(i, j.wrapping_sub(1), grid)
                + Self::dfs(i, j + 1, grid);
        }
        return max_area;
    }
}
