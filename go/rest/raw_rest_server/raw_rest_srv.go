package main

import (
   "encoding/json"
   "fmt"
   "io"
   "log"
   "net/http"
   "time"
)

type User struct {
   Username string `json:"user"`
   Password string `json:"password"`
}

var user User

var Port = ":1234"

// Data is the map that holds User records
var Data = make(map[string]string)

func defaultHandler(aWriter http.ResponseWriter, aRequest *http.Request) {
   log.Println("Serving:", aRequest.URL.Path, "from", aRequest.Host)
   aWriter.WriteHeader(http.StatusNotFound)
   Body := "Thanks for visiting!\n"
   _, _ = fmt.Fprintf(aWriter, "%s", Body)
}

func timeHandler(aWriter http.ResponseWriter, aRequest *http.Request) {
   log.Println("Serving:", aRequest.URL.Path, "from", aRequest.Host)
   timeFmt := time.Now().Format(time.RFC1123)
   Body := "The current time is: " + timeFmt + "\n"
   _, _ = fmt.Fprintf(aWriter, "%s", Body)
}

func addHandler(aWriter http.ResponseWriter, aRequest *http.Request) {
   log.Println("Serving:", aRequest.URL.Path, "from", aRequest.Host, aRequest.Method)
   if aRequest.Method != http.MethodPost {
      _, _ = fmt.Fprintf(aWriter, "%s\n", "Method not allowed!")
      http.Error(aWriter, "Error:", http.StatusMethodNotAllowed)
      return
   }

   bytesRead, err := io.ReadAll(aRequest.Body)
   if err != nil {
      http.Error(aWriter, "Error:", http.StatusBadRequest)
      return
   }

   err = json.Unmarshal(bytesRead, &user)
   if err != nil {
      log.Println("Unmarshal:", err)
      http.Error(aWriter, "Error:", http.StatusBadRequest)
      return
   }

   if user.Username == "" {
      http.Error(aWriter, "Error:", http.StatusBadRequest)
      return
   }

   Data[user.Username] = user.Password
   log.Println(Data)
   aWriter.WriteHeader(http.StatusCreated)
}

func getHandler(aWriter http.ResponseWriter, aRequest *http.Request) {
   logIt(aRequest)
   if aRequest.Method != http.MethodGet {
      _, _ = fmt.Fprintf(aWriter, "%s\n", "Method not allowed!")
      http.Error(aWriter, "Error:", http.StatusMethodNotAllowed)
      return
   }

   bytesRead, err := io.ReadAll(aRequest.Body)
   if err != nil {
      http.Error(aWriter, "ReadAll - Error", http.StatusBadRequest)
      return
   }

   err = json.Unmarshal(bytesRead, &user)
   if err != nil {
      log.Println(err)
      http.Error(aWriter, "Unmarshal - Error", http.StatusBadRequest)
      return
   }

   fmt.Println(user)

   _, ok := Data[user.Username]
   if ok && user.Username != "" {
      log.Println("Found!")
      aWriter.WriteHeader(http.StatusOK)
      _, _ = fmt.Fprintf(aWriter, "%s\n", bytesRead)
   } else {
      log.Println("Not found!")
      aWriter.WriteHeader(http.StatusNotFound)
      http.Error(aWriter, "Map - Resource not found!", http.StatusNotFound)
   }

   return
}

func logIt(aRequest *http.Request) {
   log.Println("Serving:", aRequest.URL.Path, "from", aRequest.Host, aRequest.Method)
}

func deleteHandler(aWriter http.ResponseWriter, aRequest *http.Request) {
   logIt(aRequest)
   if aRequest.Method != http.MethodDelete {
      _, _ = fmt.Fprintf(aWriter, "%s\n", "Method not allowed!")
      http.Error(aWriter, "Error:", http.StatusMethodNotAllowed)
      return
   }

   bytesRead, err := io.ReadAll(aRequest.Body)
   if err != nil {
      http.Error(aWriter, "ReadAll - Error", http.StatusBadRequest)
      return
   }

   err = json.Unmarshal(bytesRead, &user)
   if err != nil {
      log.Println(err)
      http.Error(aWriter, "Unmarshal - Error", http.StatusBadRequest)
      return
   }

   log.Println(user)

   _, ok := Data[user.Username]
   if ok && user.Username != "" {
      if user.Password == Data[user.Username] {
         delete(Data, user.Username)
         aWriter.WriteHeader(http.StatusOK)
         _, _ = fmt.Fprintf(aWriter, "%s\n", bytesRead)
         log.Println(Data)
      }
   } else {
      log.Println("User", user.Username, "Not found!")
      aWriter.WriteHeader(http.StatusNotFound)
      http.Error(aWriter, "Delete - Resource not found!", http.StatusNotFound)
   }

   return
}

func main() {
   serveMux := http.NewServeMux()
   httpServer := &http.Server{
      Addr:         Port,
      Handler:      serveMux,
      IdleTimeout:  10 * time.Second,
      ReadTimeout:  time.Second,
      WriteTimeout: time.Second,
   }

   serveMux.Handle("/time", http.HandlerFunc(timeHandler))
   serveMux.Handle("/add", http.HandlerFunc(addHandler))
   serveMux.Handle("/get", http.HandlerFunc(getHandler))
   serveMux.Handle("/delete", http.HandlerFunc(deleteHandler))
   serveMux.Handle("/", http.HandlerFunc(defaultHandler))

   fmt.Println("Ready to serve at", Port)
   err := httpServer.ListenAndServe()
   if err != nil {
      fmt.Println(err)
      return
   }
}
