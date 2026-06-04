package main

import "fmt"

func main() {
   // range works with maps as well
   aMap := make(map[string]string)
   aMap["123"] = "456"
   aMap["key"] = "A value"
   for key, val := range aMap {
      fmt.Println("key:", key, "value:", val)
   }

   for _, val := range aMap {
      fmt.Print(" # ", val)
   }

   fmt.Println()
}
