package problem

import "sort"

func carFleet(target int, position []int, speed []int) int {
	var cars = make([][2]float64, len(position))
	var result int
	var time float64
	for i := range cars {
		cars[i] = [2]float64{
			float64(position[i]),
			// time
			float64((target - position[i]) / speed[i]),
		}
	}
	sort.Slice(cars, func(i, j int) bool {
		return cars[i][0] > cars[j][0]
	})
	for _, t := range cars {
		if t[1] > time {
			time = t[1]
			result++
		}
	}
	return result
}
