package main

import (
   "fmt"
   "sync"
)

var m sync.Mutex
var w sync.WaitGroup

func function() {
   m.Lock()
   fmt.Println("Locked!")
}

func main() {
   w.Go(func() {
      function()
   })

   w.Go(func() {
      function()
   })

   w.Wait()
}
