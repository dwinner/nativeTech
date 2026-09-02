package main

import (
   "log"
   "net/http"
)

// LoggedUsersHandler returns the list of all logged-in users
func LoggedUsersHandler(writer http.ResponseWriter, request *http.Request) {
   log.Println("LoggedUsersHandler Serving:", request.URL.Path, "from", request.Host)
   var user = User{}
   err := user.FromJson(request.Body)
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

   err = SliceToJson(ReturnLoggedUsers(), writer)
   if err != nil {
      log.Println(err)
      writer.WriteHeader(http.StatusBadRequest)
      return
   }
}
