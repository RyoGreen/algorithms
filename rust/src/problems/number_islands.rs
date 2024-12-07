pub struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        fn dfs(i: i32, j: i32) {
            dfs(i + 1, j);
            dfs(i - 1, j);
            dfs(i, j + 1);
            dfs(i, j - 1);
        }
        for i in grid {
            for j in i {}
        }
        0
    }
}
