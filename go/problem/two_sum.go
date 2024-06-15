package problem

func TwoSum(nums []int, target int) [][]int {
	var result [][]int
	for i := 0; i < len(nums); i++ {
		for j := i; j < len(nums); j++ {
			if nums[i]+nums[j] == target {
				var pair = []int{nums[i], nums[j]}
				result = append(result, pair)
			}
		}
	}
	return result
}
