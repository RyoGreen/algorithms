package problem

func longestIncreasingSubsequence(nums []int) int {
	if len(nums) < 1 {
		return 0
	}

	var dp = make([]int, len(nums))
	for i := range dp {
		dp[i] = 1
	}
	for i := 1; i < len(nums); i++ {
		for j := 0; j < i; j++ {
			if nums[i] > nums[j] {
				dp[i] = max(dp[i], dp[j]+1)
			}
		}
	}

	var result int
	for _, v := range dp {
		if v > result {
			result = v
		}
	}
	return result
}

func maxProductIncreasingSubsequence(nums []int) int {
	if len(nums) < 1 {
		return 0
	}
	var dp = make([]int, len(nums))
	copy(dp, nums)
	for i := 1; i < len(nums); i++ {
		for j := 0; j < i; j++ {
			if nums[i] > nums[j] {
				dp[i] = max(dp[i], dp[j]*nums[i])
			}
		}
	}
	var result int
	for _, v := range dp {
		if v > result {
			result = v
		}
	}

	return result
}
