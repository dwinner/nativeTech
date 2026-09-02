package main

import (
   "encoding/json"
   "io"
   "log"
   "net/http"
   "time"
)

// LoginHandler is for updating the LastLogin time of a user
// And changing the Active field to true
func LoginHandler(writer http.ResponseWriter, request *http.Request) {
   log.Println("LoginHandler Serving:", request.URL.Path, "from", request.Host)
   bytes, err := io.ReadAll(request.Body)
   if err != nil {
      writer.WriteHeader(http.StatusBadRequest)
      log.Println(err)
      return
   }

   if len(bytes) == 0 {
      writer.WriteHeader(http.StatusBadRequest)
      log.Println("No input!")
      return
   }

   var user = User{}
   err = json.Unmarshal(bytes, &user)
   if err != nil {
      log.Println(err)
      writer.WriteHeader(http.StatusBadRequest)
      return
   }

   log.Println("Input user:", user)

   if !IsUserValid(user) {
      log.Println("User", user.Username, "not valid!")
      writer.WriteHeader(http.StatusBadRequest)
      return
   }

   username := FindUsername(user.Username)
   log.Println("Logging in:", username)

   username.LastLogin = time.Now().Unix()
   username.Active = 1
   if UpdateUser(username) {
      log.Println("User updated:", username)
      writer.WriteHeader(http.StatusOK)
   } else {
      log.Println("Update failed:", username)
      writer.WriteHeader(http.StatusBadRequest)
   }
}
