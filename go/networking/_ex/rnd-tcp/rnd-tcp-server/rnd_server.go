package main

import (
   "context"
   "log"
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
   ctx, stop := signal.NotifyContext(context.Background(), syscall.SIGINT, syscall.SIGTERM)

   // Getting rid of signal listener
   defer stop()

   // Wait for signal stopping
   <-ctx.Done()

   // Graceful shutdown
   _, cancelCallback := context.WithTimeout(context.Background(), ShutdownTimeout)
   defer cancelCallback()

   if err := server.Stop(); err != nil {
      log.Fatal("Error while closing the listener")
   }

   log.Println("Server stopped gracefully")
   /* Alt
      // Register signal handling
      sigChan := make(chan os.Signal, 1)
      signal.Notify(sigChan, syscall.SIGINT, syscall.SIGTERM)
      _ = <-sigChan
      server.Stop()
   */
}
