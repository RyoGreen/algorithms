package problem

import (
	"fmt"
	"testing"
)

func TestTwoSum(t *testing.T) {
	var nums = []int{2, 7, 11, 15}
	var target = 9
	var result = TwoSum(nums, target)
	if len(result) != 1 {
		t.Errorf("Expected 1, got %d", len(result))
	}
	if result[0][0] != 2 {
		t.Errorf("Expected 2, got %d", result[0][0])
	}
	if result[0][1] != 7 {
		t.Errorf("Expected 7, got %d", result[0][1])
	}
}

func BenchmarkTwoSum(b *testing.B) {
	var nums = []int{2, 7, 11, 15}
	var target = 9
	var num int
	for i := 0; i < b.N; i++ {
		num++
		TwoSum(nums, target)
	}
	fmt.Println(num)
}
