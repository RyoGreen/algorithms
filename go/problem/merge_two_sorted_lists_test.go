package problem

import "testing"

func TestmergeTwoLists(t *testing.T) {
	tests := []struct {
		name  string
		list1 *ListNode
		list2 *ListNode
		want  *ListNode
	}{
		{
			name:  "both lists empty",
			list1: nil,
			list2: nil,
			want:  nil,
		},
		{
			name:  "one list empty, other non-empty",
			list1: &ListNode{Val: 1, Next: nil},
			list2: nil,
			want:  &ListNode{Val: 1, Next: nil},
		},
		{
			name:  "both lists non-empty",
			list1: &ListNode{Val: 1, Next: &ListNode{Val: 2, Next: nil}},
			list2: &ListNode{Val: 1, Next: &ListNode{Val: 3, Next: nil}},
			want:  &ListNode{Val: 1, Next: &ListNode{Val: 1, Next: &ListNode{Val: 2, Next: &ListNode{Val: 3, Next: nil}}}},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := mergeTwoLists(tt.list1, tt.list2)
			if !isEqual(got, tt.want) {
				t.Errorf("mergeTwoLists() = %v, want %v", got, tt.want)
			}
		})
	}
}

func isEqual(l1, l2 *ListNode) bool {
	for l1 != nil && l2 != nil {
		if l1.Val != l2.Val {
			return false
		}
		l1 = l1.Next
		l2 = l2.Next
	}
	return l1 == nil && l2 == nil
}
