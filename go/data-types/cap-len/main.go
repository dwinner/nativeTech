package main

import "fmt"

func main() {
	// Only length is defined. Capacity = length
	slice := make([]int, 4)
	fmt.Println("L:", len(slice), "C:", cap(slice))

	// Initialize slice. Capacity = length
	yaSlice := []int{0, 1, 2, 3, 4}
	fmt.Println("L:", len(yaSlice), "C:", cap(yaSlice))

	// Same length and capacity
	aSlice := make([]int, 4, 4)
	fmt.Println(aSlice)

	// Add an element
	aSlice = append(aSlice, 5)
	fmt.Println(aSlice)

	// The capacity is doubled
	fmt.Println("L:", len(aSlice), "C:", cap(aSlice))

	// Now add four elements
	aSlice = append(aSlice, []int{-1, -2, -3, -4}...)
	fmt.Println(aSlice)

	// The capacity is doubled
	fmt.Println("L:", len(aSlice), "C:", cap(aSlice))
}
