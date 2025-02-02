package problem

import "testing"

func TestLongPalindrome(t *testing.T) {
	tests := []struct {
		input    string
		expected string
	}{
		{"babad", "bab"},
		{"cbbd", "bb"},
		{"a", "a"},
		{"ac", "a"},
		{"racecar", "racecar"},
		{"abccba", "abccba"},
		{"abcde", "a"},
		{"", ""},
	}

	for _, test := range tests {
		result := LongPalindrome(test.input)
		if result != test.expected {
			t.Errorf("For input %q, expected %q but got %q", test.input, test.expected, result)
		}
	}
}
