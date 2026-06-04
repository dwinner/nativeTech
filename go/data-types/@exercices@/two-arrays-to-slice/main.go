package main

import "fmt"

func concatToSlice(array1, array2 [5]int) []int {
   var result []int
   for _, val := range array1 {
      result = append(result, val)
   }

   for _, val := range array2 {
      result = append(result, val)
   }

   return result
}

func main() {
   array1 := [5]int{1, 2, 3, 4, 5}
   array2 := [5]int{6, 7, 8, 9, 10}

   slice := concatToSlice(array1, array2)

   fmt.Println(slice)
}
