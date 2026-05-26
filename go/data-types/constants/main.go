package main

import (
   "fmt"
)

type Digit int
type Power2 int

const PI = 3.1415926

const (
   C1 = "C1C1C1"
   C2 = "C2C2C2"
   C3 = "C3C3C3"
)

func main() {
   const s1 = 123
   var v1 float32 = s1 * 12
   fmt.Println(v1)
   fmt.Println(PI)

   const (
      Zero Digit = iota
      One
      Two
      Three
      Four
   )

   fmt.Println(One)
   fmt.Println(Two)

   const (
      p20 Power2 = 1 << iota
      _
      p22
      _
      p24
      _
      p26
   )

   fmt.Println("2^0:", p20)
   fmt.Println("2^2:", p22)
   fmt.Println("2^4:", p24)
   fmt.Println("2^6:", p26)
}
