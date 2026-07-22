package main

import (
   "fmt"
)

type AnotherInt int

type AllInts interface {
   ~int
}

func AddElements[T AllInts](slice []T) T {
   sum := T(0)
   for _, val := range slice {
      sum = sum + val
   }

   return sum
}

func main() {
   intSlice := []AnotherInt{0, 1, 2}
   fmt.Println(AddElements(intSlice))
}
