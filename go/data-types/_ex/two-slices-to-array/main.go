package main

import "fmt"

func concatToArray(slice1, slice2 []int) []int {
   var result []int
   for _, item := range slice1 {
      result = append(result, item)
   }

   for _, item := range slice2 {
      result = append(result, item)
   }

   return result
}

func main() {
   slice1 := []int{1, 2, 3, 4, 5}
   slice2 := []int{6, 7, 8, 9, 10}

   dynArray := concatToArray(slice1, slice2)

   fmt.Println(dynArray)
}
