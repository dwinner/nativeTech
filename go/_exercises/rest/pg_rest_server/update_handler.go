package main

import (
   "encoding/json"
   "io"
   "log"
   "net/http"
)

// UpdateHandler is for updating the data of an existing user + PUT
func UpdateHandler(writer http.ResponseWriter, request *http.Request) {
   log.Println("UpdateHandler Serving:", request.URL.Path, "from", request.Host)
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

   var users []User
   err = json.Unmarshal(bytes, &users)
   if err != nil {
      log.Println(err)
      writer.WriteHeader(http.StatusBadRequest)
      return
   }

   if !IsUserAdmin(users[0]) {
      log.Println("Command issued by non-admin user:", users[0].Username)
      writer.WriteHeader(http.StatusBadRequest)
      return
   }

   log.Println(users)
   username := FindUsername(users[1].Username)
   username.Username = users[1].Username
   username.Password = users[1].Password
   username.Admin = users[1].Admin

   if !UpdateUser(username) {
      log.Println("Update failed:", username)
      writer.WriteHeader(http.StatusBadRequest)
      return
   }

   log.Println("Update successful:", username)
   writer.WriteHeader(http.StatusOK)
}
