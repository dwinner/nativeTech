package main

import (
   "fmt"
   "io"
   "net/http"
   "net/http/httputil"
   "net/url"
   "os"
   "path/filepath"
   "strings"
   "time"
)

func main() {
   if len(os.Args) != 2 {
      fmt.Printf("Usage: %s URL\n", filepath.Base(os.Args[0]))
      return
   }

   parsedUrl, err := url.Parse(os.Args[1])
   if err != nil {
      fmt.Println("Error in parsing:", err)
      return
   }

   httpClient := &http.Client{
      Timeout: 15 * time.Second,
   }

   request, err := http.NewRequest(http.MethodGet, parsedUrl.String(), nil)
   if err != nil {
      fmt.Println("Get:", err)
      return
   }

   httpData, err := httpClient.Do(request)
   if err != nil {
      fmt.Println("Error in Do():", err)
      return
   }

   defer func(aBody io.ReadCloser) {
      err := aBody.Close()
      if err != nil {
         os.Exit(-1)
      }
   }(httpData.Body)

   fmt.Println("Status code:", httpData.Status)
   header, _ := httputil.DumpResponse(httpData, false)
   fmt.Print(string(header))

   contentType := httpData.Header.Get("Content-Type")
   characterSet := strings.SplitAfter(contentType, "charset=")
   if len(characterSet) > 1 {
      fmt.Println("Character Set:", characterSet[1])
   }

   if httpData.ContentLength == -1 {
      fmt.Println("ContentLength is unknown!")
   } else {
      fmt.Println("ContentLength:", httpData.ContentLength)
   }

   length := 0
   var buffer [1024]byte
   body := httpData.Body
   for {
      readLen, err := body.Read(buffer[0:])
      if err != nil {
         fmt.Println(err)
         break
      }

      length = length + readLen
   }

   fmt.Println("Calculated response data length:", length)
}
