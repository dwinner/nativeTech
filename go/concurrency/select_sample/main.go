package main

import (
   "fmt"
   "math/rand"
   "os"
   "strconv"
   "sync"
   "time"
)

func gen(min, max int, createNumber chan int, end chan bool) {
   time.Sleep(time.Second)
   for {
      select {
      case createNumber <- rand.Intn(max-min) + min:
      case <-end:
         fmt.Println("Ended!")
         // return
      case <-time.After(4 * time.Second):
         fmt.Println("time.After()!")
         return
      }
   }
}

func main() {
   rand.Seed(time.Now().Unix())
   createNumber := make(chan int)
   end := make(chan bool)

   if len(os.Args) != 2 {
      fmt.Println("Please give me an integer!")
      return
   }

   n, _ := strconv.Atoi(os.Args[1])
   fmt.Printf("Going to create %d random numbers.\n", n)

   var waitGrp sync.WaitGroup
   waitGrp.Go(func() {
      gen(0, 2*n, createNumber, end)
   })

   for range n {
      fmt.Printf("%d ", <-createNumber)
   }

   end <- true
   waitGrp.Wait()
   fmt.Println("Exiting...")
}
