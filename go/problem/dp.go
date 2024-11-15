package problem

func climbStairs(n int) int {
	if n <= 2 {
		return n
	}

	first, second := 1, 2
	for i := 3; i <= n; i++ {
		first, second = second, first+second
	}

	return second
}

func fibonacci(n int) int {
	if n < 2 {
		return n
	}

	first, second := 0, 1
	for i := 0; i <= n; i++ {
		second, first = first, first+second
	}

	return second
}
