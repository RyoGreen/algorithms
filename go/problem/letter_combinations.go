package problem

var phoneDigitToLettersMapping = map[rune]string{
	'2': "abc",
	'3': "def",
	'4': "ghi",
	'5': "jkl",
	'6': "mno",
	'7': "pqrs",
	'8': "tuv",
	'9': "wxyz",
}

/*
digits[i] is a digit in the range ['2', '9'].
Input: digits = "23"
Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
*/

func letterCombinations(digits string) []string {
	var result []string
	if digits == "" {
		return result
	}
	for i := 0; i < len(digits); i++ {

	}

	return result
}
