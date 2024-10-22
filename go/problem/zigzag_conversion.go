package problem

// input: PAYPALISHIRING
// output: PAHNAPLSIIGYIR
func convert(s string, numRows int) string {
	if numRows == 1 {
		return s
	}
	tmp := make([]string, numRows)

	rowIndex := 0
	down := false

	for _, char := range s {
		tmp[rowIndex] += string(char)

		if rowIndex == 0 {
			down = true
		} else if rowIndex == numRows-1 {
			down = false
		}

		if down {
			rowIndex++
		} else {
			rowIndex--
		}
	}

	result := ""
	for _, str := range tmp {
		result += str
	}
	return result
}
