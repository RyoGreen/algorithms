package problem

func isPrime(n int) bool {
	if n < 2 {
		return false
	}
	if n == 2 || n == 3 {
		return true
	}

	if n%2 == 0 || n%3 == 0 {
		return false
	}

	for i := 5; i*i <= n; i = i + 2 {
		if n%i == 0 {
			return false
		}
	}
	return true
}

func MissingNumber(nums []int) int {
	l := len(nums)
	expectedSum := l * (l + 1) / 2
	var sum int
	for _, v := range nums {
		sum += v
	}
	return expectedSum - sum
}

func Fib(n int) int {
	if n < 2 {
		return n
	}
	prev, next := 0, 1
	for i := 2; i <= n; i++ {
		prev, next = next, prev+next
	}
	return next
}
