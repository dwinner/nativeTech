package main

import (
   "fmt"
   "time"
)

func main() {
   channel1 := make(chan string)
   go func() {
      time.Sleep(3 * time.Second)
      channel1 <- "c1 OK"
   }()

   select {
   case res := <-channel1:
      fmt.Println(res)
   case <-time.After(time.Second):
      fmt.Println("timeout c1")
   }

   channel2 := make(chan string)
   go func() {
      time.Sleep(3 * time.Second)
      channel2 <- "c2 OK"
   }()

   select {
   case res := <-channel2:
      fmt.Println(res)
   case <-time.After(4 * time.Second):
      fmt.Println("timeout c2")
   }
}
