package main

import (
   "fmt"
   "log"
   "net/http"
   "pdo"
   "strconv"
   "strings"
)

const PORT = ":1234"

func defaultHandler(aResponseWriter http.ResponseWriter, aRequest *http.Request) {
   log.Println("Serving:", aRequest.URL.Path, "from", aRequest.Host)
   aResponseWriter.WriteHeader(http.StatusOK)
   body := "Thanks for visiting!\n"
   _, _ = fmt.Fprintf(aResponseWriter, "%s", body)
}

func deleteHandler(aResponseWriter http.ResponseWriter, aRequest *http.Request) {
   // Get dataset
   paramStr := strings.Split(aRequest.URL.Path, "/")
   fmt.Println("Path:", paramStr)
   if len(paramStr) < 3 {
      aResponseWriter.WriteHeader(http.StatusNotFound)
      _, _ = fmt.Fprintln(aResponseWriter, "Not found:", aRequest.URL.Path)
      return
   }

   log.Println("Serving:", aRequest.URL.Path, "from", aRequest.Host)

   dataset := paramStr[2]
   err := deleteByKey(dataset)
   if err != nil {
      fmt.Println(err)
      Body := err.Error() + "\n"
      aResponseWriter.WriteHeader(http.StatusNotFound)
      _, _ = fmt.Fprintf(aResponseWriter, "%s", Body)
      return
   }

   body := dataset + " deleted!\n"
   aResponseWriter.WriteHeader(http.StatusOK)
   _, _ = fmt.Fprintf(aResponseWriter, "%s", body)
}

func listHandler(aResponseWriter http.ResponseWriter, aRequest *http.Request) {
   log.Println("Serving:", aRequest.URL.Path, "from", aRequest.Host)
   aResponseWriter.WriteHeader(http.StatusOK)
   body := list()
   _, _ = fmt.Fprintf(aResponseWriter, "%s", body)
}

func statusHandler(aResponseWriter http.ResponseWriter, aRequest *http.Request) {
   log.Println("Serving:", aRequest.URL.Path, "from", aRequest.Host)
   aResponseWriter.WriteHeader(http.StatusOK)
   body := fmt.Sprintf("Total entries: %d\n", entryCollection.Len())
   _, _ = fmt.Fprintf(aResponseWriter, "%s", body)
}

func insertHandler(aResponseWriter http.ResponseWriter, aRequest *http.Request) {
   paramStr := strings.Split(aRequest.URL.Path, "/")
   fmt.Println("Path:", paramStr)

   if len(paramStr) < 4 {
      aResponseWriter.WriteHeader(http.StatusBadRequest)
      _, _ = fmt.Fprintln(aResponseWriter, "Not enough arguments: "+aRequest.URL.Path)
      return
   }

   dataset := paramStr[2]

   // These are string values
   dataStr := paramStr[3:]
   data := make([]float64, 0)

   for _, value := range dataStr {
      val, err := strconv.ParseFloat(value, 64)
      if err == nil {
         data = append(data, val)
      }
   }

   entry := pdo.Process(dataset, data)
   err := insert(&entry)

   if err != nil {
      aResponseWriter.WriteHeader(http.StatusNotModified)
      Body := "Failed to add record\n"
      _, _ = fmt.Fprintf(aResponseWriter, "%s", Body)
   } else {
      Body := "New record added successfully\n"
      aResponseWriter.WriteHeader(http.StatusOK)
      _, _ = fmt.Fprintf(aResponseWriter, "%s", Body)
   }

   log.Println("Serving:", aRequest.URL.Path, "from", aRequest.Host)
}

func searchHandler(aResponseWriter http.ResponseWriter, aRequest *http.Request) {
   // Get Search value from URL
   paramStr := strings.Split(aRequest.URL.Path, "/")
   fmt.Println("Path:", paramStr)

   if len(paramStr) < 3 {
      aResponseWriter.WriteHeader(http.StatusNotFound)
      _, _ = fmt.Fprintln(aResponseWriter, "Not found: "+aRequest.URL.Path)
      return
   }

   var body string
   dataset := paramStr[2]
   entry := search(dataset)
   if entry == nil {
      aResponseWriter.WriteHeader(http.StatusNotFound)
      body = "Could not be found: " + dataset + "\n"
   } else {
      aResponseWriter.WriteHeader(http.StatusOK)
      body = fmt.Sprintf("%s %d %f %f\n", entry.Name, entry.Len, entry.Mean, entry.StdDev)
   }

   log.Println("Serving:", aRequest.URL.Path, "from", aRequest.Host)
   _, _ = fmt.Fprintf(aResponseWriter, "%s", body)
}
