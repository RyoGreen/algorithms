package problem

import (
	"testing"
)

func TestThreeSum(t *testing.T) {
	tests := []struct {
		nums     []int
		expected [][]int
	}{
		{
			nums:     []int{-4, -3, -2, -2, -2, 4},
			expected: [][]int{{-2, -2, 4}},
		},
		{
			nums:     []int{-1, 0, 1, 2, -1, -4},
			expected: [][]int{{-1, -1, 2}, {-1, 0, 1}},
		},
		{
			nums:     []int{0, 1, 1},
			expected: [][]int{},
		},
		{
			nums:     []int{-2, 0, 1, 1, 2},
			expected: [][]int{{-2, 0, 2}, {-2, 1, 1}},
		},
		{
			nums:     []int{1, -1, -1, 0},
			expected: [][]int{{-1, 0, 1}},
		},
	}
	for _, test := range tests {
		result := threeSum(test.nums)
		if !equalTriplet(result, test.expected) {
			t.Errorf("For input %v, expected %v, but got %v", test.nums, test.expected, result)
		}
	}
}
