package main

import (
   "fmt"
)

type Rect struct {
   Width  int
   Height int
}

type Circle struct {
   Radius float64
}

func upcast(iface interface{}) {
   switch iface.(type) {
   case Circle:
      circle := iface.(Circle)
      fmt.Printf("Radius: '%f'\n", circle.Radius)
   case Rect:
      rect := iface.(Rect)
      fmt.Printf("Width: '%d'. Height: '%d'\n", rect.Width, rect.Height)
   default:
      fmt.Println("Unknown type switch")
   }
}

func main() {
   rect := Rect{3, 4}
   circle := Circle{3.1}
   upcast(rect)
   upcast(circle)
   upcast(3)
}
