package main

import (
   "fmt"
   "time"
)

func createSlice() []int {
   return make([]int, 1000000)
}

func getValue(aSlice []int) []int {
   returnVal := make([]int, 3)
   copy(returnVal, aSlice)
   return returnVal
}

func main() {
   for range 15 {
      message := createSlice()
      val := getValue(message)
      fmt.Print(len(val), " ")
      time.Sleep(10 * time.Millisecond)
   }
}
