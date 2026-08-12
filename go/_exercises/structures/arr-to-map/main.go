package main

import "fmt"

func main() {
   const arrLen = 10
   var intArray [arrLen]int
   for i := 0; i < arrLen; i++ {
      intArray[i] = i
   }

   intMap := make(map[int]int)
   for idx, val := range intArray {
      intMap[idx] = val
   }

   fmt.Println(intMap)
}
