package problem

import "fmt"

func MultiplicationTable() {
	for i := 1; i < 10; i++ {
		for j := 1; j < 10; j++ {
			fmt.Printf("%v", i*j)
			if j != 9 {
				fmt.Print(", ")
			}
		}
		fmt.Printf("\n")
	}
}
