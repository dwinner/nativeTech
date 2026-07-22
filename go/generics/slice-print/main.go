package main

import (
   "fmt"
)

func PrintSlice[T any](aSlice []T) {
   for _, value := range aSlice {
      fmt.Print(value, " ")
   }

   fmt.Println()
}

func main() {
   PrintSlice([]int{1, 2, 3})
   PrintSlice([]string{"a", "b", "c"})
   PrintSlice([]float64{1.2, -2.33, 4.55})
}
