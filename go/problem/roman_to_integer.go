package problem

/*
I             1
V             5
X             10
L             50
C             100
D             500
M             1000

Input: s = "MCMXCIV"
Output: 1994
*/

func RomanToInt(s string) int {
	var result int
	for i := 0; i < len(s); i++ {
		if i+1 < len(s) && helperRomanToInt(string(s[i])) < helperRomanToInt(string(s[i+1])) {
			result -= helperRomanToInt(string(s[i]))
		} else {
			result += helperRomanToInt(string(s[i]))
		}
	}
	return result
}

func helperRomanToInt(s string) int {
	switch s {
	case "I":
		return 1
	case "V":
		return 5
	case "X":
		return 10
	case "L":
		return 50
	case "C":
		return 100
	case "D":
		return 500
	case "M":
		return 1000
	default:
		return 0
	}
}
