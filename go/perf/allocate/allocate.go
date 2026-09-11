package main

import "fmt"

const Vat = 24

type Item struct {
   Description string
   Value       float64
}

func Value(price float64) float64 {
   total := price + price*Vat/100
   return total
}

func main() {
   item := Item{Description: "Keyboard", Value: 100}
   item.Value = Value(item.Value)
   fmt.Println(item)

   tP := &Item{}
   *&tP.Description = "Mouse"
   *&tP.Value = 100
   fmt.Println(tP)
}
