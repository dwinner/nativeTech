package main

import (
   "fmt"

   "example.com/mathlib"
)

func main() {
   a, b := 5, 3
   sum := mathlib.Add(a, b)
   product := mathlib.Multiply(a, b)
   fmt.Printf("%d + %d = %d", a, b, sum)
   fmt.Printf("%d * %d = %d ", a, b, product)
}
