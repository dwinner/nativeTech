package main

import (
   "fmt"
   "net/http"
   "os"
   "time"
)

func myHandler(aResponseWriter http.ResponseWriter, aRequest *http.Request) {
   _, err := fmt.Fprintf(aResponseWriter, "Serving: %s\n", aRequest.URL.Path)
   if err != nil {
      os.Exit(-1)
      return
   }

   fmt.Printf("Served: %s\n", aRequest.Host)
}

func timeHandler(aResponseWriter http.ResponseWriter, aRequest *http.Request) {
   formattedTime := time.Now().Format(time.RFC1123)
   Body := "The current time is:"
   _, _ = fmt.Fprintf(aResponseWriter, "<h1 align=\"center\">%s</h1>", Body)
   _, _ = fmt.Fprintf(aResponseWriter, "<h2 align=\"center\">%s</h2>\n", formattedTime)
   _, _ = fmt.Fprintf(aResponseWriter, "Serving: %s\n", aRequest.URL.Path)
   fmt.Printf("Served time for: %s\n", aRequest.Host)
}

func main() {
   port := ":8001"
   arguments := os.Args
   if len(arguments) != 1 {
      port = ":" + arguments[1]
   }

   fmt.Println("Using port number: ", port)
   http.HandleFunc("/time", timeHandler)
   http.HandleFunc("/", myHandler)
   err := http.ListenAndServe(port, nil)
   if err != nil {
      _, _ = fmt.Fprintln(os.Stderr, err)
      os.Exit(1)
   }
}
