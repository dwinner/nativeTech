package main

import (
   "fmt"
   "os"
   "runtime/trace"
   "time"
)

func main() {
   filename := "traceCLA.out"
   f, err := os.Create(filename)
   if err != nil {
      panic(err)
   }

   defer func(f *os.File) {
      _ = f.Close()
   }(f)

   err = trace.Start(f)
   if err != nil {
      fmt.Println(err)
      return
   }
   defer trace.Stop()

   for range 3 {
      s := make([]byte, 50000000)
      if s == nil {
         fmt.Println("Operation failed!")
      }
   }

   for range 5 {
      s := make([]byte, 100000000)
      if s == nil {
         fmt.Println("Operation failed!")
      }
      time.Sleep(time.Millisecond)
   }
}
