package main

import "fmt"

func main() {
   // Pre-allocate a slice with a capacity of 10
   mySlice := make([]int, 0, 100)

   // Append elements to the slice
   for i := range 100 {
      mySlice = append(mySlice, i)
   }

   fmt.Println(mySlice)

   // Pre-allocate a map with an initial capacity of 10
   myMap := make(map[string]int, 10)

   // Add key-value pairs to the map
   for i := range 10 {
      key := fmt.Sprintf("k%d", i)
      myMap[key] = i
   }

   fmt.Println(myMap)
}
