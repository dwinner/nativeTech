package main

import (
   "fmt"
   "io"
   "net"
   "net/http"
   "os"
   "strconv"
   "time"
)

var timeout = time.Second

func Timeout(aNetwork, aHost string) (net.Conn, error) {
   connection, dialErr := net.DialTimeout(aNetwork, aHost, timeout)
   if dialErr != nil {
      return nil, dialErr
   }

   deadlineErr := connection.SetDeadline(time.Now().Add(timeout))
   if deadlineErr != nil {
      return nil, dialErr
   }

   return connection, nil
}

func main() {
   if len(os.Args) == 1 {
      fmt.Println("Please provide a URL")
      return
   }

   if len(os.Args) == 3 {
      durationInSec, err := strconv.Atoi(os.Args[2])
      if err == nil {
         timeout = time.Duration(durationInSec) * time.Second
      }
   }

   fmt.Println("Timeout value:", timeout)
   parsedUrl := os.Args[1]
   transport := http.Transport{
      Dial: Timeout,
   }

   client := http.Client{
      Transport: &transport,
   }

   data, err := client.Get(parsedUrl)
   if err != nil {
      fmt.Println(err)
   } else {
      defer func(aBody io.ReadCloser) {
         _ = aBody.Close()
      }(data.Body)
      _, err := io.Copy(os.Stdout, data.Body)
      if err != nil {
         fmt.Println(err)
      }
   }
}
