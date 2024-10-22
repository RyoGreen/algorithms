package problem

func maxArea(height []int) int {
	var maxArea int

	left, right := 0, len(height)-1
	for left < right {
		width := right - left
		var h int
		if height[left] < height[right] {
			h = height[left]
			left++
		} else {
			h = height[right]
			right--
		}

		area := h * width
		if area > maxArea {
			maxArea = area
		}
	}

	return maxArea
}
