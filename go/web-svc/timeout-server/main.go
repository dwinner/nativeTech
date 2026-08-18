package main

import (
   "fmt"
   "net/http"
   "os"
   "time"
)

func myHandler(aWriter http.ResponseWriter, aRequest *http.Request) {
   _, _ = fmt.Fprintf(aWriter, "Serving: %s\n", aRequest.URL.Path)
   fmt.Printf("Served: %s\n", aRequest.Host)
}

func timeHandler(aWriter http.ResponseWriter, aRequest *http.Request) {
   nowTime := time.Now().Format(time.RFC1123)
   Body := "The current time is:"
   _, _ = fmt.Fprintf(aWriter, "<h1 align=\"center\">%s</h1>", Body)
   _, _ = fmt.Fprintf(aWriter, "<h2 align=\"center\">%s</h2>\n", nowTime)
   _, _ = fmt.Fprintf(aWriter, "Serving: %s\n", aRequest.URL.Path)
   fmt.Printf("Served time for: %s\n", aRequest.Host)
}

func main() {
   port := ":8001"
   arguments := os.Args
   if len(arguments) != 1 {
      port = ":" + arguments[1]
   }

   fmt.Println("Using port number: ", port)

   serveMux := http.NewServeMux()
   httpSrv := &http.Server{
      Addr:         port,
      Handler:      serveMux,
      ReadTimeout:  3 * time.Second,
      WriteTimeout: 3 * time.Second,
   }

   serveMux.HandleFunc("/time", timeHandler)
   serveMux.HandleFunc("/", myHandler)

   err := httpSrv.ListenAndServe()
   if err != nil {
      fmt.Println(err)
      return
   }
}
