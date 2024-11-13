package problem

func countIslands(grid [][]int) int {
	if len(grid) < 1 {
		return 0
	}
	var (
		rows   = len(grid)
		cols   = len(grid[0])
		island = 0
	)
	var dfs func(int, int)
	dfs = func(i, j int) {
		if i < 0 || i >= rows || j < 0 || j >= cols || grid[i][j] == 0 {
			return
		}
		grid[i][j] = 0
		dfs(i+1, j)
		dfs(i-1, j)
		dfs(i, j+1)
		dfs(i, j-1)

	}

	for i := 0; i < rows; i++ {
		for j := 0; j < cols; j++ {
			if grid[i][j] == 1 {
				island++
				dfs(i, j)
			}
		}
	}

	return island
}

func closedIsland(grid [][]int) int {
	if len(grid) < 1 {
		return 0
	}
	var (
		rows   = len(grid)
		cols   = len(grid[0])
		island = 0
	)

	var dfs func(int, int)
	dfs = func(i, j int) {

	}

	for i := 0; i < rows; i++ {
		for j := 0; j < cols; j++ {
		}
	}

	return island

}
