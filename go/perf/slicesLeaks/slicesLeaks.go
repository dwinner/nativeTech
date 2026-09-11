package main

import (
   "fmt"
   "time"
)

func createSlice() []int {
   return make([]int, 1000000)
}

func getValue(s []int) []int {
   val := s[:3]
   return val
}

func main() {
   for range 15 {
      message := createSlice()
      val := getValue(message)
      fmt.Print(len(val), " ")
      time.Sleep(10 * time.Millisecond)
   }
}
