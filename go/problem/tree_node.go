package problem

func (bst *BST) insert(val int) {
	n := &TreeNode{Val: val}
	if bst.Root == nil {
		bst.Root = n
		return
	}
	root := bst.Root
	for {
		if val < root.Val {
			if root.Left == nil {
				root.Left = &TreeNode{Val: val}
				return
			}
			root = root.Left
		} else {
			if root.Right == nil {
				root.Right = &TreeNode{Val: val}
				return
			}
			root = root.Right
		}
	}
}

func (bst *BST) search(val int) bool {
	current := bst.Root
	for {
		if current == nil {
			return false
		}
		if current.Val == val {
			return true
		}

		if current.Val > val {
			current = current.Left
		} else {
			current = current.Right
		}

	}
}
