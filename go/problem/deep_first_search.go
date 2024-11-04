package problem

func sumOfTreeNodes(root *TreeNode) int {
	if root == nil {
		return 0
	}
	return root.Val + sumOfTreeNodes(root.Left) + sumOfTreeNodes(root.Right)
}
