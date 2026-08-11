package main

import (
   "fmt"
   "os"
   "strconv"
   "sync"
)

func main() {
   count := 10
   arguments := os.Args
   if len(arguments) == 2 {
      countArg, err := strconv.Atoi(arguments[1])
      if err == nil {
         count = countArg
      }
   }

   fmt.Printf("Going to create %d goroutines.\n", count)

   var waitGrp sync.WaitGroup
   fmt.Printf("%#v\n", &waitGrp)
   for i := 0; i < count; i++ {
      waitGrp.Add(1)
      go func(x int) {
         defer waitGrp.Done()
         fmt.Printf("%d ", x)
      }(i)
   }

   fmt.Printf("%#v\n", &waitGrp)
   waitGrp.Wait()
   fmt.Println("\nExiting...")
}
