package main

import (
   "context"
   "errors"
   "fmt"
   "log"
   "net"
   "net/http"
   "os/signal"
   "sync/atomic"
   "syscall"
   "time"
)

const (
   _shutdownPeriod      = 15 * time.Second
   _shutdownHardPeriod  = 3 * time.Second
   _readinessDrainDelay = 5 * time.Second
)

var isShuttingDown atomic.Bool

func main() {
   // Setup signal context
   rootCtx, stop := signal.NotifyContext(context.Background(), syscall.SIGINT, syscall.SIGTERM)
   defer stop()

   // Readiness endpoint
   http.HandleFunc("/healthz", func(writer http.ResponseWriter, request *http.Request) {
      if isShuttingDown.Load() {
         http.Error(writer, "Shutting down", http.StatusServiceUnavailable)
         return
      }

      _, _ = fmt.Fprintln(writer, "OK")
   })

   // Sample business logic
   http.HandleFunc("/", func(writer http.ResponseWriter, request *http.Request) {
      select {
      case <-time.After(2 * time.Second):
         _, _ = fmt.Fprintln(writer, "Hello, world!")
      case <-request.Context().Done():
         http.Error(writer, "Request cancelled.", http.StatusRequestTimeout)
      }
   })

   // Ensure in-flight requests aren't canceled immediately on SIGTERM
   ongoingCtx, stopOngoingGracefully := context.WithCancel(context.Background())
   httpServer := &http.Server{
      Addr: ":8080",
      BaseContext: func(_ net.Listener) context.Context {
         return ongoingCtx
      },
   }

   go func() {
      log.Println("Server starting on :8080.")
      if err := httpServer.ListenAndServe(); err != nil && !errors.Is(err, http.ErrServerClosed) {
         panic(err)
      }
   }()

   // Wait for signal
   <-rootCtx.Done()
   stop()
   isShuttingDown.Store(true)
   log.Println("Received shutdown signal, shutting down.")

   // Give time for readiness check to propagate
   time.Sleep(_readinessDrainDelay)
   log.Println("Readiness check propagated, now waiting for ongoing requests to finish.")

   shutdownCtx, cancel := context.WithTimeout(context.Background(), _shutdownPeriod)
   defer cancel()
   err := httpServer.Shutdown(shutdownCtx)
   stopOngoingGracefully()
   if err != nil {
      log.Println("Failed to wait for ongoing requests to finish, waiting for forced cancellation.")
      time.Sleep(_shutdownHardPeriod)
   }

   log.Println("Server shut down gracefully.")
}
