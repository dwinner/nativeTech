package main

import (
   "fmt"
   "log"
   "net/http"
   "time"
)

// TimeHandler is for handling /time – it works with plain text
func TimeHandler(responseWriter http.ResponseWriter, request *http.Request) {
   log.Println("TimeHandler Serving:", request.URL.Path, "from", request.Host)
   responseWriter.WriteHeader(http.StatusOK)
   fmtTime := time.Now().Format(time.RFC1123)
   Body := "The current time is: " + fmtTime + "\n"
   _, _ = fmt.Fprintf(responseWriter, "%s", Body)
}
