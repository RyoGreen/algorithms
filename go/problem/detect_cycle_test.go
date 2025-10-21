package problem

import (
	"testing"
)

func TestDetectCycle(t *testing.T) {
	node := func(val int) *ListNode {
		return &ListNode{Val: val}
	}

	tests := []struct {
		name     string
		head     *ListNode
		expected *ListNode
		setup    func(*ListNode)
	}{
		{
			name:     "empty list",
			head:     nil,
			expected: nil,
		},
		{
			name:     "single node no cycle",
			head:     node(1),
			expected: nil,
		},
		{
			name:     "single node self-cycle",
			head:     node(1),
			expected: nil,
			setup: func(n *ListNode) {
				n.Next = n
			},
		},
		{
			name:     "3 nodes no cycle",
			head:     &ListNode{Val: 1, Next: &ListNode{Val: 2, Next: &ListNode{Val: 3, Next: nil}}},
			expected: nil,
		},
		{
			name:     "cycle in middle 2->3->4->2",
			head:     node(1),
			expected: nil,
			setup: func(n *ListNode) {
				n2 := &ListNode{Val: 2}
				n3 := &ListNode{Val: 3}
				n4 := &ListNode{Val: 4}
				n.Next = n2
				n2.Next = n3
				n3.Next = n4
				n4.Next = n2
			},
		},
		{
			name:     "cycle starts at head 1->2->3->1",
			head:     node(1),
			expected: nil, setup: func(n *ListNode) {
				n2 := &ListNode{Val: 2}
				n3 := &ListNode{Val: 3}
				n.Next = n2
				n2.Next = n3
				n3.Next = n
			},
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			if test.setup != nil && test.head != nil {
				test.setup(test.head)
			}
			result := detectCycle(test.head)
			if test.setup != nil && test.head != nil {
				if result == nil {
					t.Errorf("%s: expected non-nil cycle node, got nil", test.name)
				}
			} else {
				if result != nil {
					t.Errorf("%s: expected nil, got %v", test.name, result)
				}
			}
		})
	}
}
