package problem

import "testing"

func TestFibonacci(t *testing.T) {
	tests := []struct {
		input    int
		expected int
	}{
		{0, 0},
		{1, 1},
		{2, 1},
		{3, 2},
		{4, 3},
		{5, 5},
		{6, 8},
		{10, 55},
		{20, 6765},
	}
	for _, test := range tests {
		result := fibonacci(test.input)
		if result != test.expected {
			t.Errorf("fibonacci(%d) = %d; want %d", test.input, result, test.expected)
		}

	}
}
