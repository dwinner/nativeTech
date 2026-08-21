package main

import (
   "fmt"
   "io"
   "net/http"
   "pdo"
   "time"
)

const defaultJsonFile = "./data.json"

var entryCollection = pdo.NewEntryCollection()
var entryIndex = pdo.NewEntryIndex()
var jsonDb = pdo.NewJsonDb(defaultJsonFile)

func main() {
   err := jsonDb.Load(entryCollection)
   if err != nil && err != io.EOF {
      fmt.Println("Error:", err)
      return
   }

   entryIndex.CreateIndex(entryCollection)
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
