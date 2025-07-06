package main

import "fmt"

func main() {
	var array = []int{1, 2, 0, 1, 0}
	fmt.Println(moveZeroToTail(array))
}

func moveZeroToTail(array []int) []int {
	var (
		notZeroArray []int
		zeroArray    []int
	)
	for _, v := range array {
		if v == 0 {
			zeroArray = append(zeroArray, 0)
		} else {
			notZeroArray = append(notZeroArray, v)
		}
	}
	notZeroArray = append(notZeroArray, zeroArray...)
	return notZeroArray

}
