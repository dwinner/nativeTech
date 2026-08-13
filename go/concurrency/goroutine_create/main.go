package main

import (
	"fmt"
	"time"
)

func printMe(x int) {
	fmt.Println("*", x)
}

func main() {
	go func(x int) {
		fmt.Printf("%d ", x)
	}(10)

	go printMe(15)

	time.Sleep(time.Second)
	fmt.Println("Exiting...")
}
