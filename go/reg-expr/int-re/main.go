package main

import (
   "fmt"
   "os"
   "regexp"
)

func matchInt(anInput string) bool {
   byteArr := []byte(anInput)
   intRegExpr := regexp.MustCompile(`^[-+]?\d+$`)
   return intRegExpr.Match(byteArr)
}

func main() {
   arguments := os.Args
   if len(arguments) == 1 {
      fmt.Println("Usage: <utility> string.")
      return
   }

   arg1st := arguments[1]
   ret := matchInt(arg1st)
   fmt.Println(ret)
}
