package main

import (
   "fmt"
)

func AddInt(x, y int) int {
   for range x {
      y = y + 1
   }

   return y
}

func main() {
   fmt.Println(AddInt(5, 4))
}
