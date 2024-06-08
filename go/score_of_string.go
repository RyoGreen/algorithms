func scoreOfString(s string) {
	var sum int
	for i := 0; i < len(s)-1; i++ {
		v := int(s[i]) - int(s[i+1])
		if v > 0 {
			sum += v
		} else {
			sum += -(v)
		}
	}
}
