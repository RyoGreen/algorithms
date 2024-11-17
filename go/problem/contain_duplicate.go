package problem

func containsDuplicate(nums []int) bool {
	var m = make(map[int]struct{}, len(nums))
	for _, v := range nums {
		if _, ok := m[v]; ok {
			return true
		} else {
			m[v] = struct{}{}
		}
	}
	return false
}
