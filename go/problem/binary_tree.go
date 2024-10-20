package problem

func postorderTraversal(root *TreeNode) []int {
	if root == nil {
		return nil
	}

	var result []int
	result = append(result, postorderTraversal(root.Left)...)
	result = append(result, postorderTraversal(root.Right)...)
	result = append(result, root.Val)

	return result
}

func preorderTraversal(root *TreeNode) []int {
	if root == nil {
		return nil
	}
	var result []int
	result = append(result, root.Val)
	result = append(result, preorderTraversal(root.Left)...)
	result = append(result, preorderTraversal(root.Right)...)

	return result
}

func inorderTraversal(root *TreeNode) []int {
	if root == nil {
		return nil
	}
	var result []int
	result = append(result, inorderTraversal(root.Left)...)
	result = append(result, root.Val)
	result = append(result, inorderTraversal(root.Right)...)
	return result
}

func isValidBST(root *TreeNode) bool {
	var nums []int
	nums = append(nums, inorderTraversal(root.Left)...)
	nums = append(nums, root.Val)
	nums = append(nums, inorderTraversal(root.Right)...)

	for i := 0; i < len(nums)-1; i++ {
		if nums[i] >= nums[i+1] {
			return false
		}
	}

	return true
}
