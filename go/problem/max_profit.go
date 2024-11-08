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
