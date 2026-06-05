package main

import (
   "fmt"
   "regexp"
)

func main() {
   inputStr := "seafood"
   const rePattern = `foo.*`
   matched, err := regexp.MatchString(rePattern, inputStr)
   fmt.Println(matched, err)

   matched, err = regexp.Match(rePattern, []byte(inputStr))
   fmt.Println(matched, err)
}
