package problem

/*
I             1
V             5
X             10
L             50
C             100
D             500
M             1000
*/

func RomanToInt(s string) int {
	var result int
	for _, v := range []rune(s) {
		result += helperRomanToInt(string(v))
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
