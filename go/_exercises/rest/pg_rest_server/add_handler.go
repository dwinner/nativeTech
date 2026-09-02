package main

import (
   "encoding/json"
   "io"
   "log"
   "net/http"
)

// AddHandler is for adding a new user
func AddHandler(responseWriter http.ResponseWriter, request *http.Request) {
   log.Println("AddHandler Serving:", request.URL.Path, "from", request.Host)
   bytes, err := io.ReadAll(request.Body)
   if err != nil {
      responseWriter.WriteHeader(http.StatusBadRequest)
      log.Println(err)
      return
   }

   if len(bytes) == 0 {
      responseWriter.WriteHeader(http.StatusBadRequest)
      log.Println("No input!")
      return
   }

   // We read two structures as an array:
   // 1. The user issuing the command
   // 2. The user to be added
   var users []User
   err = json.Unmarshal(bytes, &users)
   if err != nil {
      log.Println(err)
      responseWriter.WriteHeader(http.StatusBadRequest)
      return
   }

   log.Println(users)

   if len(users) < 2 && !IsUserAdmin(users[0]) {
      log.Println("Command issued by non-admin user:", users[0].Username)
      responseWriter.WriteHeader(http.StatusBadRequest)
      return
   }

   result := InsertUser(users[1])
   if !result {
      responseWriter.WriteHeader(http.StatusBadRequest)
   }
}
