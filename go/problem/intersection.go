package problem

func FindIntersection(nums1 []int, nums2 []int) []int {
	var result []int
	m := make(map[int]bool, len(nums1))
	for _, v := range nums1 {
		m[v] = true
	}

	for _, v := range nums2 {
		if m[v] {
			result = append(result, v)
			delete(m, v)
		}
	}

	return result
}
