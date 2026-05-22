package main

import (
   "fmt"
   "math/rand"
   "os"
   "strconv"
   "time"
)

var MIN = 0
var MAX = 94

func random(min, max int) int {
   return rand.Intn(max-min) + min
}

func getString(len int64) string {
   temp := ""
   startChar := "!"
   var i int64 = 1
   for {
      myRand := random(MIN, MAX)
      newChar := string(startChar[0] + byte(myRand))
      temp = temp + newChar
      if i == len {
         break
      }

      i++
   }

   return temp
}

func main() {
   LENGTH := int64(8)

   arguments := os.Args
   switch len(arguments) {
   case 2:
      i64Val, err := strconv.ParseInt(os.Args[1], 10, 64)
      if err == nil {
         LENGTH = i64Val
      }

      if LENGTH <= 0 {
         LENGTH = 8
      }
   default:
      fmt.Println("Using default values...")
   }

   SEED := time.Now().Unix()
   rand.Seed(SEED)
   fmt.Println(getString(LENGTH))
}
