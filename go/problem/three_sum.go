package problem

import "sort"

func threeSum(nums []int) [][]int {
	var result [][]int

	sort.Ints(nums)
	for i := 0; i < len(nums)-2; i++ {
		if nums[i] >= 1 {
			break
		}

		if i > 0 && nums[i] == nums[i-1] {
			continue
		}

		l, r := i+1, len(nums)-1
		for l < r {
			sum := nums[i] + nums[l] + nums[r]
			switch {
			case sum == 0:
				result = append(result, []int{nums[i], nums[l], nums[r]})
				for l++; l < r && nums[l] == nums[l-1]; l++ {
				}
				for r--; l < r && nums[r] == nums[r+1]; r-- {
				}

			case sum > 0:
				r--
			default:
				l++
			}
		}
	}

	return result
}
