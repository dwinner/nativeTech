package main

import (
   "fmt"
   "sync"
   "time"
)

var Password *secret
var waitGroup sync.WaitGroup

type secret struct {
   RwGuard  sync.RWMutex
   password string
}

func Change(aPassword string) {
   if Password == nil {
      fmt.Println("Password is nil!")
      return
   }

   fmt.Println("Change() function")
   Password.RwGuard.Lock()
   fmt.Println("Change() Locked")
   time.Sleep(4 * time.Second)
   Password.password = aPassword
   Password.RwGuard.Unlock()
   fmt.Println("Change() UnLocked")
}

func show() {
   defer waitGroup.Done()
   defer Password.RwGuard.RUnlock()
   Password.RwGuard.RLock()
   fmt.Println("Show function locked!")
   time.Sleep(2 * time.Second)
   fmt.Println("Pass value:", Password.password)
}

func main() {
   Password = &secret{password: "myPass"}
   for range 3 {
      waitGroup.Add(1)
      go show()
   }

   waitGroup.Go(func() {
      Change("123456")
   })

   waitGroup.Go(func() {
      Change("54321")
   })

   waitGroup.Wait()

   // Direct access to Password.password
   fmt.Println("Current password value:", Password.password)
}
