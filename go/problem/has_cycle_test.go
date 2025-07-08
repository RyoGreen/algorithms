package problem

import "testing"

func TestHasCycle(t *testing.T) {
	tests := []struct {
		head     *ListNode
		expected bool
	}{
		{
			head:     &ListNode{Val: 1, Next: &ListNode{Val: 2, Next: &ListNode{Val: 3, Next: nil}}},
			expected: false,
		},
		{
			head:     &ListNode{Val: 1, Next: &ListNode{Val: 2, Next: &ListNode{Val: 3, Next: nil}}},
			expected: false,
		},
	}

	for _, test := range tests {
		t.Run("", func(t *testing.T) {
			result := hasCycle(test.head)
			if result != test.expected {
				t.Errorf("hasCycle(%v) = %v; expected %v", test.head, result, test.expected)
			}
		})
	}
}
