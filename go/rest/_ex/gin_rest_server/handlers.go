package main

import (
   "encoding/json"
   "fmt"
   "io"
   "log"
   "net/http"
   "time"

   "github.com/gin-gonic/gin"
)

type User struct {
   Username string `json:"user"`
   Password string `json:"password"`
}

var user User

// Data is the map that holds User records
var Data = make(map[string]string)

func handleDefault(aGinCtx *gin.Context) {
   request := aGinCtx.Request
   writer := aGinCtx.Writer
   log.Println("Serving:", request.URL.Path, "from", request.Host)
   writer.WriteHeader(http.StatusNotFound)
   body := "Thanks for visiting!\n"
   _, _ = fmt.Fprintf(writer, "%s", body)
}

func handleTime(aGinCtx *gin.Context) {
   request := aGinCtx.Request
   writer := aGinCtx.Writer
   log.Println("Serving:", request.URL.Path, "from", request.Host)
   timeFmt := time.Now().Format(time.RFC1123)
   Body := "The current time is: " + timeFmt + "\n"
   _, _ = fmt.Fprintf(writer, "%s", Body)
}

func handleAdd(aGinCtx *gin.Context) {
   request := aGinCtx.Request
   writer := aGinCtx.Writer
   log.Println("Serving:", request.URL.Path, "from", request.Host, request.Method)
   bytesRead, err := io.ReadAll(request.Body)
   if err != nil {
      http.Error(writer, "Error:", http.StatusBadRequest)
      return
   }

   err = json.Unmarshal(bytesRead, &user)
   if err != nil {
      log.Println("Unmarshal:", err)
      http.Error(writer, "Error:", http.StatusBadRequest)
      return
   }

   if user.Username == "" {
      http.Error(writer, "Error:", http.StatusBadRequest)
      return
   }

   Data[user.Username] = user.Password
   log.Println(Data)
   writer.WriteHeader(http.StatusCreated)
}

func handleGet(aGinCtx *gin.Context) {
   request := aGinCtx.Request
   writer := aGinCtx.Writer
   log.Println("Serving:", request.URL.Path, "from", request.Host, request.Method)
   bytesRead, err := io.ReadAll(request.Body)
   if err != nil {
      http.Error(writer, "ReadAll - Error", http.StatusBadRequest)
      return
   }

   err = json.Unmarshal(bytesRead, &user)
   if err != nil {
      log.Println(err)
      http.Error(writer, "Unmarshal - Error", http.StatusBadRequest)
      return
   }

   fmt.Println(user)

   _, ok := Data[user.Username]
   if ok && user.Username != "" {
      log.Println("Found!")
      writer.WriteHeader(http.StatusOK)
      _, _ = fmt.Fprintf(writer, "%s\n", bytesRead)
   } else {
      log.Println("Not found!")
      writer.WriteHeader(http.StatusNotFound)
      http.Error(writer, "Map - Resource not found!", http.StatusNotFound)
   }

   return
}

func handleDelete(aGinCtx *gin.Context) {
   request := aGinCtx.Request
   writer := aGinCtx.Writer
   log.Println("Serving:", request.URL.Path, "from", request.Host, request.Method)
   bytesRead, err := io.ReadAll(request.Body)
   if err != nil {
      http.Error(writer, "ReadAll - Error", http.StatusBadRequest)
      return
   }

   err = json.Unmarshal(bytesRead, &user)
   if err != nil {
      log.Println(err)
      http.Error(writer, "Unmarshal - Error", http.StatusBadRequest)
      return
   }

   log.Println(user)

   _, ok := Data[user.Username]
   if ok && user.Username != "" {
      if user.Password == Data[user.Username] {
         delete(Data, user.Username)
         writer.WriteHeader(http.StatusOK)
         _, _ = fmt.Fprintf(writer, "%s\n", bytesRead)
         log.Println(Data)
      }
   } else {
      log.Println("User", user.Username, "Not found!")
      writer.WriteHeader(http.StatusNotFound)
      http.Error(writer, "Delete - Resource not found!", http.StatusNotFound)
   }

   return
}
