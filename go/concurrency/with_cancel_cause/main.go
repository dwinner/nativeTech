package main

import (
   "context"
   "errors"
   "fmt"
   "time"
)

func main() {
   ctx := context.Background()
   ctx, cancel := context.WithCancelCause(ctx)
   cancel(errors.New("cancelled by timeout"))
   err := takingTooLong(ctx)
   if err != nil {
      fmt.Println(err)
      return
   }
}

func takingTooLong(aContext context.Context) error {
   select {
   case <-time.After(3 * time.Second):
      fmt.Println("Done!")
      return nil
   case <-aContext.Done():
      fmt.Println("Cancelled!")
      return context.Cause(aContext)
   }
}
