package main

import (
   "fmt"
   "os"
   "runtime"
   "strconv"
   "sync"
   "time"
)

type Client struct {
   id      int
   integer int
}

type Result struct {
   job    Client
   square int
}

var size = runtime.GOMAXPROCS(0)
var clients = make(chan Client, size)
var data = make(chan Result, size)

func worker(aWaitGroup *sync.WaitGroup) {
   for client := range clients {
      square := client.integer * client.integer
      output := Result{client, square}
      data <- output
      time.Sleep(time.Second)
   }

   aWaitGroup.Done()
}

func create(aClientNum int) {
   for id := range aClientNum {
      client := Client{id, id}
      clients <- client
   }

   close(clients)
}

func main() {
   if len(os.Args) != 3 {
      fmt.Println("Need #jobs and #workers!")
      return
   }

   nJobs, err := strconv.Atoi(os.Args[1])
   if err != nil {
      fmt.Println(err)
      return
   }

   nWorkers, err := strconv.Atoi(os.Args[2])
   if err != nil {
      fmt.Println(err)
      return
   }

   go create(nJobs)

   finished := make(chan any)
   go func() {
      for item := range data {
         fmt.Printf("Client ID: %d\tint: ", item.job.id)
         fmt.Printf("%d\tsquare: %d\n", item.job.integer, item.square)
      }

      finished <- true
   }()

   var waitGroup sync.WaitGroup
   for range nWorkers {
      waitGroup.Add(1)
      go worker(&waitGroup)
   }

   waitGroup.Wait()
   close(data)

   fmt.Printf("Finished: %v\n", <-finished)
}
