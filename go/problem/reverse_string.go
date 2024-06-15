package problem

func ReverseString(s string) string {
	r := []rune(s)
	length := len(r) / 2
	for i := 0; i < length; i++ {
		r[i], r[len(r)-i-1] = r[len(r)-i-1], r[i]
	}
	return string(r)
}
