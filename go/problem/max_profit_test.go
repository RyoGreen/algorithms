package problem

import "testing"

func TestMaxProfit(t *testing.T) {
	tests := []struct {
		name     string
		prices   []int
		expected int
	}{
		{
			name:     "Example 1 - Max profit possible",
			prices:   []int{7, 1, 5, 3, 6, 4},
			expected: 5, // Buy at 1, sell at 6
		},
		{
			name:     "Example 2 - No profit possible",
			prices:   []int{7, 6, 4, 3, 1},
			expected: 0, // No profit since prices are descending
		},
		{
			name:     "Single day price - No profit possible",
			prices:   []int{5},
			expected: 0, // Only one price, no transaction possible
		},
		{
			name:     "Two days - Profit possible",
			prices:   []int{1, 5},
			expected: 4, // Buy at 1, sell at 5
		},
		{
			name:     "Two days - No profit possible",
			prices:   []int{5, 1},
			expected: 0, // Prices drop, no profit possible
		},
		{
			name:     "Multiple days with varying prices",
			prices:   []int{3, 8, 2, 5, 7, 1, 4},
			expected: 5, // Buy at 2, sell at 7
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := maxProfit(tt.prices)
			if result != tt.expected {
				t.Errorf("expected %d, got %d", tt.expected, result)
			}
		})
	}
}
