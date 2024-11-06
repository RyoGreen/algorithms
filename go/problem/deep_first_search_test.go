package problem

import "testing"

func TestHasPathSum(t *testing.T) {
	tests := []struct {
		root      *TreeNode
		targetSum int
		expected  bool
	}{
		{
			root:      &TreeNode{Val: 5, Left: &TreeNode{Val: 4, Left: &TreeNode{Val: 11, Left: &TreeNode{Val: 7}, Right: &TreeNode{Val: 2}}, Right: nil}, Right: &TreeNode{Val: 8, Left: nil, Right: &TreeNode{Val: 4, Right: &TreeNode{Val: 1}}}},
			targetSum: 22,
			expected:  true,
		},
		{
			root:      &TreeNode{Val: 1, Left: &TreeNode{Val: 2}, Right: &TreeNode{Val: 3}},
			targetSum: 5,
			expected:  false,
		},
		{
			root:      &TreeNode{Val: 1, Left: &TreeNode{Val: 2}, Right: &TreeNode{Val: 3}},
			targetSum: 4,
			expected:  true,
		},
		{
			root:      nil,
			targetSum: 0,
			expected:  true,
		},
		{
			root:      nil,
			targetSum: 1,
			expected:  false,
		},
	}

	for _, test := range tests {
		t.Run("", func(t *testing.T) {
			result := hasPathSum(test.root, test.targetSum)
			if result != test.expected {
				t.Errorf("hasPathSum(%v, %d) = %v; expected %v", test.root, test.targetSum, result, test.expected)
			}
		})
	}
}
