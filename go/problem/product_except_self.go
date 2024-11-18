package problem

func productExceptSelf(nums []int) []int {
	var (
		length = len(nums)
		result = make([]int, length, length)
		left   = 1
		right  = 1
	)

	for i := 0; i < length; i++ {
		result[i] = left
		left *= nums[i]
	}

	for i := length - 1; i >= 0; i-- {
		result[i] *= right
		right *= nums[i]
	}

	return result
}
