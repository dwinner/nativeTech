package main

import (
   "context"
   "fmt"
   "os"
   "strconv"
   "time"
)

// The f1 function creates and executes a goroutine
// The time.Sleep() call simulates the time it would take a real goroutine
// to do its job - in this case it is 4 seconds. If the c1 context calls
// the Done() function in less than 4 seconds, the goroutine will not have
// enough time to finish.
func f1(duration int) {
   cancelCtx := context.Background()

   // WithCancel returns a copy of parent context with a new Done channel
   cancelCtx, cancel := context.WithCancel(cancelCtx)
   defer cancel()

   go func() {
      time.Sleep(4 * time.Second)
      cancel()
   }()

   select {
   case <-cancelCtx.Done():
      fmt.Println("f1() Done:", cancelCtx.Err())
      return
   case r := <-time.After(time.Duration(duration) * time.Second):
      fmt.Println("f1():", r)
   }

   return
}

func f2(duration int) {
   bgCtx := context.Background()
   bgCtx, cancel := context.WithTimeout(bgCtx, time.Duration(duration)*time.Second)
   defer cancel()

   go func() {
      time.Sleep(4 * time.Second)
      cancel()
   }()

   select {
   case <-bgCtx.Done():
      fmt.Println("f2() Done:", bgCtx.Err())
      return
   case r := <-time.After(time.Duration(duration) * time.Second):
      fmt.Println("f2():", r)
   }

   return
}

func f3(duration int) {
   bgCtx := context.Background()
   deadline := time.Now().Add(time.Duration(2*duration) * time.Second)
   bgCtx, cancel := context.WithDeadline(bgCtx, deadline)
   defer cancel()

   go func() {
      time.Sleep(4 * time.Second)
      cancel()
   }()

   select {
   case <-bgCtx.Done():
      fmt.Println("f3() Done:", bgCtx.Err())
      return
   case r := <-time.After(time.Duration(duration) * time.Second):
      fmt.Println("f3():", r)
   }

   return
}

func main() {
   if len(os.Args) != 2 {
      fmt.Println("Need a delay!")
      return
   }

   delay, err := strconv.Atoi(os.Args[1])
   if err != nil {
      fmt.Println(err)
      return
   }

   fmt.Println("Delay:", delay)

   f1(delay)
   f2(delay)
   f3(delay)
}
