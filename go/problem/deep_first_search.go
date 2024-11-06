package problem

import "fmt"

func sumOfTreeNodes(root *TreeNode) int {
	if root == nil {
		return 0
	}
	return root.Val + sumOfTreeNodes(root.Left) + sumOfTreeNodes(root.Right)
}

func hasPathSum(root *TreeNode, targetSum int) bool {
	if root == nil {
		return targetSum == 0
	}
	targetSum -= root.Val
	if root.Left == nil && root.Right == nil {
		return targetSum == 0
	}

	return hasPathSum(root.Right, targetSum) || hasPathSum(root.Left, targetSum)
}

