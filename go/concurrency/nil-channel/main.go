package main

import (
   "fmt"
   "math/rand"
   "sync"
   "time"
)

var waitGroup sync.WaitGroup

func add(aChannel chan int) {
   sum := 0
   timer := time.NewTimer(time.Second)
   for {
      select {
      case input := <-aChannel:
         sum = sum + input
      case <-timer.C:
         aChannel = nil
         fmt.Println(sum)
         waitGroup.Done()
      }
   }
}

func send(aChannel chan int) {
   for {
      aChannel <- rand.Intn(10)
   }
}

func main() {
   intChannel := make(chan int)
   rand.Seed(time.Now().Unix())

   waitGroup.Add(1)
   go add(intChannel)
   go send(intChannel)
   waitGroup.Wait()
}
