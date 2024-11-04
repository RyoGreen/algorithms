package problem

import (
	"math"
	"sort"
)

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

func threeSumClosest(nums []int, target int) int {
	var result = nums[0] + nums[1] + nums[2]
	sort.Ints(nums)
	for i := 0; i < len(nums)-2; i++ {
		if i > 0 && nums[i] == nums[i-1] {
			continue
		}
		l, r := i+1, len(nums)-1
		for l < r {
			sum := nums[i] + nums[l] + nums[r]

			if math.Abs(float64(target-sum)) < math.Abs(float64(target-result)) {
				result = sum
			}

			if sum == target {
				return sum
			} else if sum < target {
				l++
			} else {
				r--
			}
		}
	}

	return result
}
