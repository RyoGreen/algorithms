package problem

func mergeTwoLists(list1 *ListNode, list2 *ListNode) *ListNode {
	list1.Next = list2
	return list1
}
