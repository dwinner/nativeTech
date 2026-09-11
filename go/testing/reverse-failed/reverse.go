package main

// BUG This version has bugs

import (
   "fmt"
)

func Reverse1(aStr string) []byte {
   strArray := []byte(aStr)
   rev := make([]byte, len(strArray))
   inputLen := len(strArray)
   for i := range inputLen {
      rev[i] = strArray[inputLen-1-i]
   }

   return rev
}

func Reverse2(aStr string) string {
   byteArray := []byte(aStr)
   for i, j := 0, len(byteArray)-1; i < len(byteArray)/2; i, j = i+1, j-1 {
      byteArray[i], byteArray[j] = byteArray[j], byteArray[i]
   }

   return string(byteArray)
}

func main() {
   str := "1234567890"
   fmt.Println(string(Reverse1(str)))
   fmt.Println(Reverse2(str))
}
