package problem

func LongPalindrome(s string) string {
	if len(s) == 0 {
		return ""
	}

	var start, end int

	helper := func(s string, left, right int) (int, int) {
		for left >= 0 && right < len(s) && s[left] == s[right] {
			left--
			right++
		}
		return left + 1, right - 1
	}

	for i := 0; i < len(s); i++ {
		leftOne, rightOne := helper(s, i, i)
		leftTwo, rightTwo := helper(s, i, i+1)

		if rightOne-leftOne > end-start {
			start, end = leftOne, rightOne
		}
		if rightTwo-leftTwo > end-start {
			start, end = leftTwo, rightTwo
		}

	}

	return s[start : end+1]
}
