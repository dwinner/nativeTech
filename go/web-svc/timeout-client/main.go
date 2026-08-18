package main

import (
   "context"
   "fmt"
   "io"
   "net/http"
   "os"
   "strconv"
   "time"
)

var delay = 5

func main() {
   if len(os.Args) == 1 {
      fmt.Println("Need a URL and a delay!")
      os.Exit(1)
   }

   url := os.Args[1]
   if len(os.Args) == 3 {
      durationInSec, err := strconv.Atoi(os.Args[2])
      if err != nil {
         fmt.Println(err)
         return
      }

      delay = durationInSec
   }

   fmt.Println("Delay:", delay)

   ctx, cancelCallback := context.WithTimeout(context.Background(), time.Second*time.Duration(delay))
   defer cancelCallback()

   req, err := http.NewRequestWithContext(ctx, http.MethodGet, url, nil)
   if err != nil {
      fmt.Println(err)
      return
   }

   res, err := http.DefaultClient.Do(req.WithContext(ctx))
   if err != nil {
      fmt.Println(err)
      return
   }

   defer func(aBody io.ReadCloser) {
      _ = aBody.Close()
   }(res.Body)

   body, err := io.ReadAll(res.Body)
   if err != nil {
      fmt.Println(err)
      return
   }

   fmt.Println(string(body))
}
