package main

import (
   "fmt"
   "runtime"
)

func printAlloc() {
   var memStats runtime.MemStats
   runtime.ReadMemStats(&memStats)
   fmt.Printf("%d KB\n", memStats.Alloc/1024)
}

func main() {
   n := 2000000
   m := make(map[int][128]byte)
   printAlloc()

   for i := range n {
      m[i] = [128]byte{}
   }

   printAlloc()

   for i := range n {
      delete(m, i)
   }

   runtime.GC()
   printAlloc()
   runtime.KeepAlive(m)

   m = nil
   runtime.GC()
   printAlloc()
}
