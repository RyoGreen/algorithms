package problem

// input: abcdabcde
// output: 5
func lengthOfLongestSubstring(s string) int {
	m := make(map[rune]int)
	result := 0
	startIndex := 0

	for currentIndex, c := range s {
		if idx, found := m[c]; found && idx >= startIndex {
			startIndex = idx + 1
		}
		m[c] = currentIndex
		result = max(result, currentIndex-startIndex+1)
	}

	return result
}
