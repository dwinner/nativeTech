package main

import (
   "fmt"
   "sync"
   "sync/atomic"
)

type atomCounter struct {
   val int64
}

func (anAtomCounter *atomCounter) Value() int64 {
   return atomic.LoadInt64(&anAtomCounter.val)
}

func main() {
   X := 100
   Y := 4
   var waitGroup sync.WaitGroup
   counter := atomCounter{}
   for range X {
      waitGroup.Go(func() {
         for range Y {
            atomic.AddInt64(&counter.val, 1)
         }
      })
   }

   waitGroup.Wait()
   fmt.Println(counter.Value())
}
