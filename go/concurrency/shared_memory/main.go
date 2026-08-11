package main

import (
   "fmt"
   "os"
   "sync"
   "time"
)

var Password = secret{password: "myPassword"}

type secret struct {
   RwGuard  sync.RWMutex
   password string
}

func Change(aSecret *secret, aPassword string) {
   aSecret.RwGuard.Lock()
   fmt.Println("LChange")
   time.Sleep(10 * time.Second)
   aSecret.password = aPassword
   aSecret.RwGuard.Unlock()
}

func show(aSecret *secret) string {
   aSecret.RwGuard.RLock()
   fmt.Print("show")
   time.Sleep(3 * time.Second)
   defer aSecret.RwGuard.RUnlock()
   return aSecret.password
}

func showWithLock(aSecret *secret) string {
   aSecret.RwGuard.Lock()
   fmt.Println("showWithLock")
   time.Sleep(3 * time.Second)
   defer aSecret.RwGuard.Unlock()
   return aSecret.password
}

func main() {
   var showFunction = func(secret *secret) string { return "" }
   if len(os.Args) != 2 {
      fmt.Println("Using sync.RWMutex!")
      showFunction = show
   } else {
      fmt.Println("Using sync.Mutex!")
      showFunction = showWithLock
   }

   var waitGrp sync.WaitGroup

   fmt.Println("Pass:", showFunction(&Password))
   for range 15 {
      waitGrp.Go(func() {
         fmt.Println("Go Pass:", showFunction(&Password))
      })
   }

   go func() {
      waitGrp.Add(1)
      defer waitGrp.Done()
      Change(&Password, "123456")
   }()

   waitGrp.Wait()
   fmt.Println("Pass:", showFunction(&Password))
}
