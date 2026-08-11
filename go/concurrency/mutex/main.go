package main

import (
   "fmt"
   "os"
   "strconv"
   "sync"
   "time"
)

var mutex sync.Mutex
var sharedInt int

func change() {
   mutex.Lock()
   defer mutex.Unlock()
   time.Sleep(time.Second)
   sharedInt = sharedInt + 1
   if sharedInt == 10 {
      sharedInt = 0
      fmt.Print("* ")
   }
}

func read() int {
   mutex.Lock()
   a := sharedInt
   defer mutex.Unlock()
   return a
}

func main() {
   if len(os.Args) != 2 {
      fmt.Println("Please give me an integer!")
      return
   }

   gortNum, err := strconv.Atoi(os.Args[1])
   if err != nil {
      fmt.Println(err)
      return
   }

   var waitGroup sync.WaitGroup
   fmt.Printf("%d ", read())
   for range gortNum {
      waitGroup.Go(func() {
         change()
         fmt.Printf("-> %d", read())
      })
   }

   waitGroup.Wait()
   fmt.Printf("-> %d\n", read())
}
