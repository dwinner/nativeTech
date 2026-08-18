package main

import (
   "fmt"
   "io"
   "net/http"
   "os"
   "path/filepath"
)

func main() {
   if len(os.Args) != 2 {
      fmt.Printf("Usage: %s URL\n", filepath.Base(os.Args[0]))
      return
   }

   url := os.Args[1]
   data, err := http.Get(url)
   if err != nil {
      os.Exit(-1)
   }

   defer func(aBody io.ReadCloser) {
      err := aBody.Close()
      if err != nil {
         os.Exit(-1)
      }
   }(data.Body)

   _, err = io.Copy(os.Stdout, data.Body)
   if err != nil {
      fmt.Println(err)
      return
   }
}
