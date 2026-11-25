package main

import (
   "log"
   "net/http"
   "strconv"

   "github.com/gorilla/mux"
)

// DeleteHandler is for deleting an existing user + DELETE
func DeleteHandler(responseWriter http.ResponseWriter, request *http.Request) {
   log.Println("DeleteHandler Serving:", request.URL.Path, "from", request.Host)

   // Get the ID of the user to be deleted
   id, ok := mux.Vars(request)["id"]
   if !ok {
      log.Println("ID value not set!")
      responseWriter.WriteHeader(http.StatusNotFound)
      return
   }

   var user = User{}
   err := user.FromJson(request.Body)
   if err != nil {
      log.Println(err)
      responseWriter.WriteHeader(http.StatusBadRequest)
      return
   }

   if !IsUserAdmin(user) {
      log.Println("User", user.Username, "is not admin!")
      responseWriter.WriteHeader(http.StatusBadRequest)
      return
   }

   intID, err := strconv.Atoi(id)
   if err != nil {
      log.Println("id", err)
      return
   }

   userId := FindUserId(intID)
   if userId.Username != "" {
      log.Println("About to delete:", userId)
      deleted := DeleteUser(intID)
      if deleted {
         log.Println("User deleted:", id)
         responseWriter.WriteHeader(http.StatusOK)
         return
      }

      log.Println("User ID not found:", id)
      responseWriter.WriteHeader(http.StatusNotFound)
   }

   responseWriter.WriteHeader(http.StatusNotFound)
}
