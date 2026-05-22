package main

import (
   "crypto/rand"
   "encoding/base64"
   "fmt"
   "os"
   "strconv"
)

// This function returns (secure) random bytes
func generateBytes(numSeed int64) ([]byte, error) {
   bytes := make([]byte, numSeed)
   _, err := rand.Read(bytes)
   if err != nil {
      return nil, err
   }

   return bytes, nil
}

func generatePass(numSeed int64) (string, error) {
   bytes, err := generateBytes(numSeed)
   if err != nil {
      return "", err
   }

   return base64.URLEncoding.EncodeToString(bytes), nil
}

func main() {
   const DefaultPassLen = 8
   var passLen int64 = DefaultPassLen
   arguments := os.Args
   switch len(arguments) {
   case 2:
      intInput, err := strconv.ParseInt(os.Args[1], 10, 64)
      if err == nil {
         passLen = intInput
      }

      if passLen <= 0 {
         passLen = DefaultPassLen
      }
   default:
      fmt.Println("Using default values!")
   }

   myPass, err := generatePass(passLen)
   if err != nil {
      fmt.Println(err)
      return
   }

   fmt.Println(myPass[0:passLen])
}
