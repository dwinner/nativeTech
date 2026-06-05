package main

import (
   "fmt"
   "os"
   "regexp"
)

func matchNameSur(anInput string) bool {
   inputArr := []byte(anInput)
   nameRe := regexp.MustCompile(`^[A-Z][a-z]*$`)
   return nameRe.Match(inputArr)
}

func main() {
   arguments := os.Args
   if len(arguments) <= 1 {
      fmt.Println("Usage: <utility> string.")
      return
   }

   for _, arg := range arguments[1:] {
      ret := matchNameSur(arg)
      if ret {
         fmt.Printf("The name '%s' is Ok\n", arg)
      }
   }
}
