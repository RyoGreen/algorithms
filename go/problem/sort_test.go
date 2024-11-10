package problem

import (
	"reflect"
	"testing"
)

var sortTestDatas = []struct {
	name     string
	input    []int
	expected []int
}{
	{
		name:     "Basic sorting",
		input:    []int{3, 1, 2, 5, 4},
		expected: []int{1, 2, 3, 4, 5},
	},
	{
		name:     "Empty array",
		input:    []int{},
		expected: []int{},
	},
	{
		name:     "Single element",
		input:    []int{1},
		expected: []int{1},
	},
	{
		name:     "Already sorted",
		input:    []int{1, 2, 3, 4, 5},
		expected: []int{1, 2, 3, 4, 5},
	},
	{
		name:     "Reverse sorted",
		input:    []int{5, 4, 3, 2, 1},
		expected: []int{1, 2, 3, 4, 5},
	},
	{
		name:     "Array with duplicates",
		input:    []int{4, 5, 3, 5, 4},
		expected: []int{3, 4, 4, 5, 5},
	},
}

func TestQuickSort(t *testing.T) {
	for _, test := range sortTestDatas {
		t.Run(test.name, func(t *testing.T) {
			result := quickSort(test.input)
			if !reflect.DeepEqual(result, test.expected) {
				t.Errorf("Expected %v, but got %v", test.expected, result)
			}
		})
	}
}
