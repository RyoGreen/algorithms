package problem

func mergeTwoLists(list1 *ListNode, list2 *ListNode) *ListNode {
	var dummy = &ListNode{}
	var current = dummy
	for list1 != nil && list2 != nil {
		if list1.Val > list2.Val {
			current.Next = list2
			list2 = list2.Next
		} else {
			current.Next = list1
			list1 = list1.Next
		}
		current = current.Next
	}
	if list1 == nil {
		current.Next = list2
	} else {
		current.Next = list1
	}

	return dummy.Next
}
