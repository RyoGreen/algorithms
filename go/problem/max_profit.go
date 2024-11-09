package problem

/*
Input: prices = [7,1,5,3,6,4]
Output: 5
*/
func maxProfit(prices []int) int {
	if len(prices) < 1 {
		return 0
	}
	var (
		result    int
		buyAmount = prices[0]
	)
	for _, v := range prices {
		if v < buyAmount {
			buyAmount = v
		} else {
			profit := v - buyAmount
			if profit > result {
				result = profit
			}
		}
	}

	return result
}

/*
Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
Output: 6
*/
func maxSubArray(nums []int) int {
	var (
		maxSum = nums[0]
		curSum = 0
	)

	for _, v := range nums {
		if curSum < 0 {
			curSum = 0
		}
		curSum += v
		maxSum = max(maxSum, curSum)
	}

	return maxSum
}
