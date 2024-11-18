package problem

func productExceptSelf(nums []int) []int {
	var (
		length = len(nums)
		result = make([]int, length, length)
	)

	for i := range nums {
		var tmp int = 1
		for ii, v := range nums {
			if ii != i {
				tmp *= v
			}
		}
		result[i] = tmp
	}
	return result
}
