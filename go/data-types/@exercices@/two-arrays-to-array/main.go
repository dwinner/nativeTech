package main

import "fmt"

func concatToArray(array1, array2 [5]int) [10]int {
   var result [10]int
   startIdx := 0

   for _, val := range array1 {
      result[startIdx] = val
      startIdx++
   }

   for _, val := range array2 {
      result[startIdx] = val
      startIdx++
   }

   return result
}

func main() {
   array1 := [5]int{1, 2, 3, 4, 5}
   array2 := [5]int{6, 7, 8, 9, 10}

   newArray := concatToArray(array1, array2)

   fmt.Println(newArray)
}
