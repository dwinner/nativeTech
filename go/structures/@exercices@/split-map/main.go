package main

import (
   "fmt"
   "strconv"
)

func main() {
   strMap := make(map[string]int)
   for i := 0; i < 5; i++ {
      str := strconv.Itoa(i)
      strMap[str] = i
   }

   var strSlice []string
   var intSlice []int
   strSlice = make([]string, 0)
   intSlice = make([]int, 0)
   for key, val := range strMap {
      strSlice = append(strSlice, key)
      intSlice = append(intSlice, val)
   }

   fmt.Println(strSlice)
   fmt.Println(intSlice)
}
