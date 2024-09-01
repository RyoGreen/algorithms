package problem

import (
	"reflect"
	"testing"
)

func TestMergeInterval(t *testing.T) {
	tests := []struct {
		Name     string
		Input    [][]int
		Expected [][]int
	}{
		{
			Name:     "empty input",
			Input:    [][]int{},
			Expected: [][]int{},
		},
		{
			Name:     "single interval",
			Input:    [][]int{{5, 10}},
			Expected: [][]int{{5, 10}},
		},
		{
			Name:     "already merged intervals",
			Input:    [][]int{{1, 3}, {4, 6}, {7, 9}},
			Expected: [][]int{{1, 3}, {4, 6}, {7, 9}},
		},
		{
			Name:     "merge overlapping intervals",
			Input:    [][]int{{1, 3}, {2, 6}, {8, 10}, {15, 18}},
			Expected: [][]int{{1, 6}, {8, 10}, {15, 18}},
		},
		{
			Name:     "merge touching intervals",
			Input:    [][]int{{1, 2}, {2, 3}, {3, 4}},
			Expected: [][]int{{1, 4}},
		},
		{
			Name:     "merge multiple intervals into one",
			Input:    [][]int{{1, 4}, {2, 5}, {3, 6}, {7, 8}},
			Expected: [][]int{{1, 6}, {7, 8}},
		},
		{
			Name:     "disjoint intervals",
			Input:    [][]int{{1, 2}, {3, 5}, {6, 7}, {8, 10}},
			Expected: [][]int{{1, 2}, {3, 5}, {6, 7}, {8, 10}},
		},
		{
			Name:     "overlap with a single interval",
			Input:    [][]int{{1, 10}, {2, 3}, {4, 5}, {6, 7}},
			Expected: [][]int{{1, 10}},
		},
		{
			Name:     "unsorted intervals",
			Input:    [][]int{{8, 10}, {1, 3}, {2, 6}, {15, 18}},
			Expected: [][]int{{1, 6}, {8, 10}, {15, 18}},
		},
		{
			Name:     "nested intervals",
			Input:    [][]int{{1, 10}, {2, 3}, {4, 5}, {6, 7}, {8, 9}},
			Expected: [][]int{{1, 10}},
		},
		{
			Name:     "intervals with same start or end",
			Input:    [][]int{{1, 3}, {3, 6}, {6, 8}, {8, 10}},
			Expected: [][]int{{1, 10}},
		},
	}

	for _, test := range tests {
		t.Run(test.Name, func(t *testing.T) {
			got := MergeInterval(test.Input)
			if !reflect.DeepEqual(got, test.Expected) {
				t.Errorf("failed merge interval expected: %v, but got: %v", test.Expected, got)
			}
		})
	}
}
