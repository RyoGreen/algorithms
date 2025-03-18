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

	for i := range rows {
		for j := range cols {
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

	rows, cols := len(grid), len(grid[0])
	island := 0

	visit := make(map[int]map[int]bool)
	for i := range rows {
		visit[i] = make(map[int]bool)
	}

	var dfs func(int, int) bool
	dfs = func(i, j int) bool {
		if i < 0 || j < 0 || i == rows || j == cols {
			return false
		}
		if grid[i][j] == 1 || visit[i][j] {
			return true
		}
		visit[i][j] = true

		up := dfs(i-1, j)
		down := dfs(i+1, j)
		left := dfs(i, j-1)
		right := dfs(i, j+1)

		return up && down && left && right
	}

	for i := 1; i < rows-1; i++ {
		for j := 1; j < cols-1; j++ {
			if grid[i][j] == 0 && !visit[i][j] {
				if dfs(i, j) {
					island++
				}
			}
		}
	}

	return island
}
