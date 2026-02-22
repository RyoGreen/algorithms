pub struct Solution;

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut grid = grid;
        let mut max_area = 0;

        for i in 0..m {
            for j in 0..n {
                let area = Self::dfs(i as i32, j as i32, &mut grid);
                max_area = max_area.max(area);
            }
        }
        return max_area;
    }
    fn dfs(i: i32, j: i32, grid: &mut Vec<Vec<i32>>) -> i32 {
        if i >= grid.len() as i32
            || j >= grid[0].len() as i32
            || i < 0
            || j < 0
            || grid[i as usize][j as usize] == 0
        {
            return 0;
        }
        grid[i as usize][j as usize] = 0;
        1 + Self::dfs(i - 1, j, grid)
            + Self::dfs(i + 1, j, grid)
            + Self::dfs(i, j - 1, grid)
            + Self::dfs(i, j + 1, grid)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area_of_island() {
        let grid = vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        ];
        assert_eq!(Solution::max_area_of_island(grid), 6);

        let grid = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        assert_eq!(Solution::max_area_of_island(grid), 0);

        let grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(Solution::max_area_of_island(grid), 1);

        let grid = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];
        assert_eq!(Solution::max_area_of_island(grid), 9);

        let grid = vec![
            vec![1, 0, 0, 1],
            vec![1, 0, 0, 1],
            vec![0, 0, 0, 0],
            vec![1, 1, 0, 0],
        ];
        assert_eq!(Solution::max_area_of_island(grid), 2);
    }
}
