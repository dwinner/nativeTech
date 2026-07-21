package main

import (
   "fmt"
)

func Same[T comparable](cmp1, cmp2 T) bool {
   if cmp1 == cmp2 {
      return true
   }

   return false
}

func main() {
   fmt.Println("4 = 3 is", Same(4, 3))
   fmt.Println("aa = aa is", Same("aa", "aa"))
   fmt.Println("4.1 = 4.15 is", Same(4.1, 4.15))
}
