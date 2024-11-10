package problem

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
