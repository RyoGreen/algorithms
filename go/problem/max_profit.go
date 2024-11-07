package problem

/*
Input: prices = [7,1,5,3,6,4]
Output: 5
*/
func maxProfit(prices []int) int {
	var result int
	for i := 0; i < len(prices)-1; i++ {
		for j := i + 1; j < len(prices); j++ {
			profit := prices[j] - prices[i]
			if profit > result {
				result = profit
			}
		}
	}
	return result
}
