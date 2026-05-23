package main

import (
   "fmt"
   "math/rand"
   "os"
   "strconv"
   "time"
)

func random(min, max int) int {
   return rand.Intn(max-min) + min
}

func main() {
   MIN := 0
   MAX := 100
   TOTAL := 100
   SEED := time.Now().Unix()

   arguments := os.Args[1:]

   for i, arg := range arguments {
      intVal, err := strconv.Atoi(arg)
      if err != nil {
         continue
      }

      switch i {
      case 0:
         MIN = intVal
         MAX = MIN + 100
      case 1:
         MAX = intVal
      case 2:
         TOTAL = intVal
      case 3:
         SEED = int64(intVal)
      }
   }

   switch len(arguments) {
   case 0:
      fmt.Println("Usage: ./randomNumbers MIN MAX TOTAL SEED")
      fmt.Println("Using all default values!")
   case 1, 2, 3:
      fmt.Println("Using some default values!")
   default:
      fmt.Println("Using given values!")
   }

   rand.Seed(SEED)
   for i := 0; i < TOTAL; i++ {
      myRand := random(MIN, MAX)
      fmt.Print(myRand)
      fmt.Print(" ")
   }

   fmt.Println()
}
