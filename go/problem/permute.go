package problem

func Permute(nums []int) (result [][]int) {
	result = make([][]int, 0)
	result = pHelper(nums, 0, result)
	return result
}

func pHelper(nums []int, start int, result [][]int) [][]int {
	if start == len(nums)-1 {
		tmp := make([]int, len(nums))
		copy(tmp, nums)
		result = append(result, tmp)
		return result
	}

	for i := start; i < len(nums); i++ {
		nums[start], nums[i] = nums[i], nums[start]

		result = pHelper(nums, start+1, result)
		nums[start], nums[i] = nums[i], nums[start]
	}

	return result
}
