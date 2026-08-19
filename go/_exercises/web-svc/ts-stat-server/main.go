package main

import (
   "fmt"
   "io"
   "net/http"
   "pdo"
   "time"
)

func main() {
   err := readJsonFile(pdo.Jsonfile)
   if err != nil && err != io.EOF {
      fmt.Println("Error:", err)
      return
   }

   createIndex()
   serveMux := http.NewServeMux()
   httpServer := &http.Server{
      Addr:         PORT,
      Handler:      serveMux,
      IdleTimeout:  10 * time.Second,
      ReadTimeout:  time.Second,
      WriteTimeout: time.Second,
   }

   serveMux.Handle("/list", http.HandlerFunc(listHandler))
   serveMux.Handle("/insert/", http.HandlerFunc(insertHandler))
   serveMux.Handle("/insert", http.HandlerFunc(insertHandler))
   serveMux.Handle("/search", http.HandlerFunc(searchHandler))
   serveMux.Handle("/search/", http.HandlerFunc(searchHandler))
   serveMux.Handle("/delete/", http.HandlerFunc(deleteHandler))
   serveMux.Handle("/status", http.HandlerFunc(statusHandler))
   serveMux.Handle("/", http.HandlerFunc(defaultHandler))

   fmt.Println("Ready to serve at", PORT)
   err = httpServer.ListenAndServe()
   if err != nil {
      fmt.Println(err)
      return
   }
}
