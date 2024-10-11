package pastinterview

func GetMinTime(n int, request string, mingap int) int {
	lastProcessed := make(map[rune]int)
	time := 0

	for _, r := range request {
		if lastTime, ok := lastProcessed[r]; ok {
			if time-lastTime < mingap {
				time = lastTime + mingap
			}
		}
		lastProcessed[r] = time
		time++
	}
	return time
}
