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
   if len(arguments) <= 1 {
      fmt.Println("Usage: <utility> string.")
      return
   }

   results := make([]string, len(arguments)-1)
   for idx, val := range arguments[1:] {
      isValid := matchInt(val)
      var result string
      if isValid {
         result = fmt.Sprintf("Value '%s' is valid", val)
      } else {
         result = fmt.Sprintf("Value '%s' is not valid", val)
      }

      results[idx] = result
   }

   for _, val := range results {
      fmt.Println(val)
   }
}
