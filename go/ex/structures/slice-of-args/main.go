package main

import (
   "fmt"
   "os"
)

type ArgRec struct {
   Index int
   Value string
}

func main() {
   args := os.Args
   var arrOfRecs []ArgRec
   for index, value := range args {
      rec := ArgRec{index, value}
      arrOfRecs = append(arrOfRecs, rec)
   }

   for _, value := range arrOfRecs {
      fmt.Println(value)
   }
}
