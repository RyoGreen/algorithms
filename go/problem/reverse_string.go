package problem

import (
	"math"
	"strconv"
)

func ReverseString(s string) string {
	r := []rune(s)
	length := len(r) / 2
	for i := 0; i < length; i++ {
		r[i], r[len(r)-i-1] = r[len(r)-i-1], r[i]
	}
	return string(r)
}

func ReverseInt(x int) int {
	var (
		s      string
		isPlus bool
	)
	s = strconv.Itoa(int(math.Abs(float64(x))))
	if x > 0 {
		isPlus = true
	}
	r := []rune(s)
	length := len(r) / 2
	for i := 0; i < length; i++ {
		r[i], r[len(r)-i-1] = r[len(r)-i-1], r[i]
	}

	result, _ := strconv.Atoi(string(r))
	if !isPlus {
		return -result
	}
	return result
}
