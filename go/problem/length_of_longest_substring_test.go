package problem

import "testing"

func TestLengthOfLongestSubstring(t *testing.T) {
	tests := []struct {
		input    string
		expected int
	}{
		{"abcabcbb", 3},
		{"bbbbb", 1},
		{"pwwkew", 3},
		{"", 0},
		{"a", 1},
		{"abcdabcde", 5},
		{"dvdf", 3},
		{"abcdeabcd", 5},
		{"abcabcdabcdef", 6},
		{"abcaxyzpcabcd", 10},
	}

	for _, test := range tests {
		result := lengthOfLongestSubstring(test.input)
		if result != test.expected {
			t.Errorf("lengthOfLongestSubstring(%q) = %d; expected %d", test.input, result, test.expected)
		}
	}
}
