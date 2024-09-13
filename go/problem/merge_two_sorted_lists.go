package problem

type ListNode struct {
	Val  int
	Next *ListNode
}

func mergeTwoLists(list1 *ListNode, list2 *ListNode) *ListNode {
	s := []int{}
	for {
		s = append(s, list1.Val)
		if list1.Next == nil {
			break
		}
	}
	s2 := []int{}
	for {
		s = append(s, list1.Val)
		if list1.Next == nil {
			break
		}

	}

	return list1
}
