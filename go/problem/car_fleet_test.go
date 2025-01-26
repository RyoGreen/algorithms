package problem

import "testing"

func TestCarFleet(t *testing.T) {
	tests := []struct {
		name     string
		expected int
		target   int
		position []int
		speed    []int
	}{
		{
			name:     "Test_1",
			expected: 3,
			target:   12,
			position: []int{10, 8, 0, 5, 3},
			speed:    []int{2, 4, 1, 1, 3},
		},
		{
			name:     "Test_2",
			expected: 1,
			target:   10,
			position: []int{3},
			speed:    []int{3},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := carFleet(tt.target, tt.position, tt.speed)
			if result != tt.expected {
				t.Errorf("expected %d, got %d", tt.expected, result)
			}
		})
	}
}
