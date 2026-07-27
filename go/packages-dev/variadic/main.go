package main

import (
   "fmt"
   "os"
)

func addFloats(aMessage string, aSlice ...float64) float64 {
   fmt.Println(aMessage)
   sum := float64(0)
   for _, value := range aSlice {
      sum = sum + value
   }

   aSlice[0] = -1000
   return sum
}

func everything(anInput ...interface{}) {
   fmt.Println(anInput)
}

func main() {
   sum := addFloats("Adding numbers...", 1.1, 2.12, 3.14, 4, 5, -1, 10)
   fmt.Println("Sum:", sum)
   slice := []float64{1.1, 2.12, 3.14}
   sum = addFloats("Adding numbers...", slice...)
   fmt.Println("Sum:", sum)
   everything(slice)

   // Cannot directly pass []string as []interface{}
   // You have to convert it first!
   empty := make([]interface{}, len(os.Args[1:]))
   for idx, val := range os.Args[1:] {
      empty[idx] = val
   }

   everything(empty...)

   // There is a slightly different way to do the conversion
   arguments := os.Args[1:]
   empty = make([]interface{}, len(arguments))
   for i := range arguments {
      empty[i] = arguments[i]
   }

   everything(empty...)

   // This will work!
   str := []string{"One", "Two", "Three"}
   everything(str, str, str)
}
