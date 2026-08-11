package main

import (
   "fmt"
)

func printer(aWriteChan chan<- bool, times int) {
   for range times {
      aWriteChan <- true
   }

   close(aWriteChan)
}

func main() {
   // This is an unbuffered channel
   var ch = make(chan bool)

   // Write 5 values to channel with a single goroutine
   go printer(ch, 5)

   // IMPORTANT: As the channel ch is closed,
   // the range loop is going to exit on its own.
   for val := range ch {
      fmt.Print(val, " ")
   }

   fmt.Println()

   for range 15 {
      fmt.Print(<-ch, " ")
   }

   fmt.Println()
}
