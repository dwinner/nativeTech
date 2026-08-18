package main

import (
   "fmt"
   "io"
   "net/http"
   "os"

   "golang.org/x/sync/errgroup"
)

func main() {
   if len(os.Args) == 1 {
      fmt.Println("Not enough arguments!")
      return
   }

   errorGrp := new(errgroup.Group)
   for _, url := range os.Args[1:] {
      lUrl := url
      errorGrp.Go(func() error {
         response, err := http.Get(lUrl)
         if err != nil {
            return err
         }

         defer func(aBody io.ReadCloser) {
            _ = aBody.Close()
         }(response.Body)

         fmt.Println(lUrl, "is OK.")
         return nil
      })
   }

   err := errorGrp.Wait()
   if err != nil {
      fmt.Println("Error:", err)
      return
   }

   fmt.Println("Everything went fine!")
}
