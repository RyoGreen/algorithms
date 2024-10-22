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

func quickSort(n []int) []int {
	if len(n) < 2 {
		return n
	}

	index := len(n) / 2

	p := n[index]
	var right, left []int
	for i, v := range n {
		if i == index {
			continue
		}
		if v >= p {
			right = append(right, v)
		} else {
			left = append(left, v)
		}
	}

	right = quickSort(right)
	left = quickSort(left)

	var result []int
	result = append(left, p)
	result = append(result, right...)

	return result
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
