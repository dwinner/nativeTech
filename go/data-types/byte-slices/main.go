package main

import "fmt"

func main() {
	// Byte slice
	slice := make([]byte, 12)
	fmt.Println("Byte slice:", slice)
	slice = []byte("Byte slice €")
	fmt.Println("Byte slice:", slice)

	// Print byte slice contents as text
	fmt.Printf("Byte slice as text: %s\n", slice)
	fmt.Println("Byte slice as text:", string(slice))

	// Length of slice
	fmt.Println("Length of slice:", len(slice))
}
