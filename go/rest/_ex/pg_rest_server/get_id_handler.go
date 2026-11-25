package main

import (
   "encoding/json"
   "io"
   "log"
   "net/http"

   "github.com/gorilla/mux"
)

// GetIdHandler returns the ID of an existing user
func GetIdHandler(writer http.ResponseWriter, request *http.Request) {
   log.Println("GetIDHandler Serving:", request.URL.Path, "from", request.Host)

   username, ok := mux.Vars(request)["username"]
   if !ok {
      log.Println("Username value not set!")
      writer.WriteHeader(http.StatusNotFound)
      return
   }

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
   if !IsUserAdmin(user) {
      log.Println("User", user.Username, "not an admin!")
      writer.WriteHeader(http.StatusBadRequest)
      return
   }

   userName := FindUsername(username)
   if userName.Id != 0 {
      writer.WriteHeader(http.StatusFound)
      err := userName.ToJson(writer)
      if err != nil {
         writer.WriteHeader(http.StatusBadRequest)
         log.Println(err)
         return
      }

      log.Println("GetIDHandler() terminated OK.")
   } else {
      writer.WriteHeader(http.StatusNotFound)
      log.Println("User " + username + " not found!")
   }
}
