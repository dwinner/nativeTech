package main

import (
   "fmt"
   "os"
   "sync"
)

func main() {
   count := 20
   fmt.Printf("Going to create %d goroutines.\n", count)
   flag := true
   if len(os.Args) == 1 {
      flag = false
   }

   var waitGroup sync.WaitGroup

   fmt.Printf("%#v\n", &waitGroup)
   for idx := range count {
      waitGroup.Add(1)
      go func(aValue int) {
         defer waitGroup.Done()
         fmt.Printf("%d ", aValue)
      }(idx)
   }

   // More Add() calls
   if flag {
      waitGroup.Add(1)
   } else {
      // More Done() calls
      waitGroup.Done()
   }

   fmt.Printf("%#v\n", &waitGroup)
   waitGroup.Wait()
   fmt.Println("\nExiting...")
}
