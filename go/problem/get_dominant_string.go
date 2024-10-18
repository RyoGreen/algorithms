package problem

func getDominantStringCount(s string) int {
	n := len(s)
	dominantCount := 0

	for length := 2; length <= n; length += 2 {
		charCount := make([]int, 25)

		for start := 0; start <= n-length; start++ {
			if start == 0 {
				for i := 0; i < length; i++ {
					charCount[s[start+i]-'a']++
				}
			} else {
				charCount[s[start-1]-'a']--
				charCount[s[start+length-1]-'a']++
			}

			targetFrequency := length / 2
			for _, count := range charCount {
				if count == targetFrequency {
					dominantCount++
					break
				}
			}
		}
	}

	return dominantCount
}
