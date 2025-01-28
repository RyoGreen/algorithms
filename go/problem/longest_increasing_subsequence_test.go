package problem

import "testing"

func TestMaxProductIncreasingSubsequence(t *testing.T) {
	tests := []struct {
		input    []int
		expected int
	}{
		{[]int{3, 2, 5, 10, 7}, 150},
		{[]int{1, 2, 3, 4, 5}, 120},
		{[]int{10, 5, 4, 3}, 10},
		{[]int{3, 6, 2, 10, 8, 1}, 180},
	}

	for _, test := range tests {
		result := maxProductIncreasingSubsequence(test.input)
		if result != test.expected {
			t.Errorf("maxProductIncreasingSubsequence(%v) = %d; expected %d", test.input, result, test.expected)
		}
	}
}
