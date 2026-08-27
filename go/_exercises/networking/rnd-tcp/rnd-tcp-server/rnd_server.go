package main

import (
   "os"
   "os/signal"
   "syscall"
   "time"
)

const (
   ListenAddr      = ":1234"
   ShutdownTimeout = 30 * time.Second
)

func main() {
   server := NewServer(ListenAddr)


   // Register signal handling
   sigChan := make(chan os.Signal, 1)
   signal.Notify(sigChan, syscall.SIGINT, syscall.SIGTERM)

   // Wait for signal stopping
   _ = <-sigChan
   server.Stop()
}
