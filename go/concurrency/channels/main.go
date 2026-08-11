package main

import (
   "fmt"
   "sync"
)

func writeToChannel(aWriteChannel chan int, x int) {
   aWriteChannel <- x
   close(aWriteChannel)
}

func printer(aWriteChan chan bool) {
   aWriteChan <- true
}

func main() {
   intChan := make(chan int, 1)

   var waitGroup sync.WaitGroup
   waitGroup.Add(1)
   go func(aChannel chan int) {
      defer waitGroup.Done()
      writeToChannel(aChannel, 10)
      fmt.Println("Exit.")
   }(intChan)

   fmt.Println("Read:", <-intChan)
   _, ok := <-intChan
   if ok {
      fmt.Println("Channel is open!")
   } else {
      fmt.Println("Channel is closed!")
   }

   waitGroup.Wait()

   var ch = make(chan bool)
   for range 5 {
      go printer(ch)
   }

   // Range on channels
   // IMPORTANT: As the channel ch is not closed,
   // the range loop does not exit by its own.
   n := 0
   for i := range ch {
      fmt.Println(i)
      if i == true {
         n++
      }

      if n > 2 {
         fmt.Println("n:", n)
         close(ch)
         break
      }
   }

   for range 5 {
      fmt.Println(<-ch)
   }
}
