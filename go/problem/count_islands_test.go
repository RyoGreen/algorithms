package problem

import "testing"

func TestCountIslands(t *testing.T) {
	tests := []struct {
		name string
		grid [][]int
		want int
	}{
		{
			name: "No islands",
			grid: [][]int{
				{0, 0, 0},
				{0, 0, 0},
				{0, 0, 0},
			},
			want: 0,
		},
		{
			name: "One island",
			grid: [][]int{
				{1, 1, 0},
				{1, 0, 0},
				{0, 0, 0},
			},
			want: 1,
		},
		{
			name: "Multiple islands",
			grid: [][]int{
				{1, 0, 0},
				{0, 1, 1},
				{0, 0, 1},
			},
			want: 2,
		},
		{
			name: "All land",
			grid: [][]int{
				{1, 1, 1},
				{1, 1, 1},
				{1, 1, 1},
			},
			want: 1,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := countIslands(tt.grid)
			if got != tt.want {
				t.Errorf("CountIslands() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestClosedIsland(t *testing.T) {
	tests := []struct {
		name string
		grid [][]int
		want int
	}{
		{
			name: "Multiple closed islands",
			grid: [][]int{
				{1, 1, 1, 1, 1},
				{1, 0, 0, 0, 1},
				{1, 0, 1, 0, 1},
				{1, 0, 0, 0, 1},
				{1, 1, 1, 1, 1},
			},
			want: 1,
		},
		{
			name: "Single closed island",
			grid: [][]int{
				{1, 1, 1, 1},
				{1, 0, 1, 1},
				{1, 1, 1, 1},
			},
			want: 1,
		},
		{
			name: "No closed island",
			grid: [][]int{
				{1, 0, 1},
				{0, 1, 0},
				{1, 0, 1},
			},
			want: 0,
		},
		{
			name: "Empty grid",
			grid: [][]int{},
			want: 0,
		},
		{
			name: "All 1s grid",
			grid: [][]int{
				{1, 1, 1},
				{1, 1, 1},
				{1, 1, 1},
			},
			want: 0,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := closedIsland(tt.grid)
			if got != tt.want {
				t.Errorf("closedIsland() = %d, want %d", got, tt.want)
			}
		})
	}
}
