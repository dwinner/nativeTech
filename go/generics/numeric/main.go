package main

import (
   "fmt"
)

type Numeric interface {
   int | int8 | int16 | int32 | int64 | float64
}

func Add[T Numeric](aNum1, aNum2 T) T {
   return aNum1 + aNum2
}

func main() {
   fmt.Println("4 + 3 =", Add(4, 3))
   fmt.Println("4.1 + 3.2 =", Add(4.1, 3.2))
}
