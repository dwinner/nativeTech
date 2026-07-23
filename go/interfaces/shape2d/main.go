package main

import (
   "fmt"
   "math"
)

type Shape2D interface {
   Perimeter() float64
}

type Circle struct {
   Radius float64
}

func (aCircle Circle) Perimeter() float64 {
   return 2 * math.Pi * aCircle.Radius
}

func main() {
   circle := Circle{Radius: 1.5}
   fmt.Printf("R %.2f -> Perimeter %.3f \n", circle.Radius, circle.Perimeter())

   _, ok := interface{}(circle).(Shape2D)
   if ok {
      fmt.Println("a is a Shape2D!")
   }

   i := 12
   _, ok = interface{}(i).(Shape2D)
   if ok {
      fmt.Println("i is a Shape2D!")
   }
}
