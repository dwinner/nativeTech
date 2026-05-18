package main

import (
	"fmt"
	"os"
	"strconv"
)

func main() {
	arguments := os.Args
	if len(arguments) == 1 {
		fmt.Println("Need one or more arguments!")
		return
	}

	var fMin, fMax float64
	var initialized = 0
	for i := 1; i < len(arguments); i++ {
		n, err := strconv.ParseFloat(arguments[i], 64)
		if err != nil {
			continue
		}

		if initialized == 0 {
			fMin = n
			fMax = n
			initialized = 1
			continue
		}

		if n < fMin {
			fMin = n
		}

		if n > fMax {
			fMax = n
		}
	}

	fmt.Println("Min:", fMin)
	fmt.Println("Max:", fMax)
}
