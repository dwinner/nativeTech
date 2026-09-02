package main

import (
   "encoding/json"
   "io"
   "log"
   "net/http"
)

// LogoutHandler is for logging out a user
// And changing the Active field to false
func LogoutHandler(writer http.ResponseWriter, request *http.Request) {
   log.Println("LogoutHandler Serving:", request.URL.Path, "from", request.Host)

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

   if !IsUserValid(user) {
      log.Println("User", user.Username, "exists!")
      writer.WriteHeader(http.StatusBadRequest)
      return
   }

   username := FindUsername(user.Username)
   log.Println("Logging out:", username.Username)
   username.Active = 0
   if UpdateUser(username) {
      log.Println("User updated:", username)
      writer.WriteHeader(http.StatusOK)
   } else {
      log.Println("Update failed:", username)
      writer.WriteHeader(http.StatusBadRequest)
   }
}
