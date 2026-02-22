pub struct Solution;

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }
        let rows = grid.len();
        let cols = grid[0].len();
        let mut result: i32 = 0;
        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == '1' {
                    result += 1;
                    Solution::dfs(&mut grid, i, j, rows, cols);
                }
            }
        }
        result
    }

    fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize, rows: usize, cols: usize) {
        if i >= rows || j >= cols || grid[i][j] == '0' {
            return;
        }

        grid[i][j] = '0';
        if i > 0 {
            Solution::dfs(grid, i - 1, j, rows, cols);
        }
        if i + 1 < rows {
            Solution::dfs(grid, i + 1, j, rows, cols);
        }
        if j > 0 {
            Solution::dfs(grid, i, j - 1, rows, cols);
        }
        if j + 1 < cols {
            Solution::dfs(grid, i, j + 1, rows, cols);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_islands_empty_grid() {
        let grid = vec![];
        assert_eq!(Solution::num_islands(grid), 0);
    }

    #[test]
    fn test_num_islands_single_land() {
        let grid = vec![vec!['1']];
        assert_eq!(Solution::num_islands(grid), 1);
    }

    #[test]
    fn test_num_islands_single_water() {
        let grid = vec![vec!['0']];
        assert_eq!(Solution::num_islands(grid), 0);
    }

    #[test]
    fn test_num_islands_multiple_islands() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        assert_eq!(Solution::num_islands(grid), 3);
    }

    #[test]
    fn test_num_islands_large_single_island() {
        let grid = vec![
            vec!['1', '1', '1'],
            vec!['1', '1', '1'],
            vec!['1', '1', '1'],
        ];
        assert_eq!(Solution::num_islands(grid), 1);
    }

    #[test]
    fn test_num_islands_no_island() {
        let grid = vec![
            vec!['0', '0', '0'],
            vec!['0', '0', '0'],
            vec!['0', '0', '0'],
        ];
        assert_eq!(Solution::num_islands(grid), 0);
    }
}
