package main

import (
   "fmt"
)

type Numeric interface {
   int | int8 | int16 | int32 | int64 | float64
}

func Print(iface interface{}) {
   // type switch
   switch iface.(type) {
   case int:
      fmt.Println(iface.(int) + 1)
   case float64:
      fmt.Println(iface.(float64) + 1)
   default:
      fmt.Println("Unknown data type!")
   }
}

func PrintGenerics[T any](anAny T) {
   fmt.Println(anAny)
}

func PrintNumeric[T Numeric](aNumber T) {
   fmt.Println(aNumber + 1)
}

func main() {
   Print(12)
   Print(-1.23)
   Print("Hi!")

   PrintGenerics(1)
   PrintGenerics("a")
   PrintGenerics(-2.33)

   PrintNumeric(1)
   PrintNumeric(-2.33)
}
