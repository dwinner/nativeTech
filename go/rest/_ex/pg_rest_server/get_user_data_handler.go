package main

import (
   "log"
   "net/http"
   "strconv"

   "github.com/gorilla/mux"
)

// GetUserDataHandler + GET returns the full record of a user
func GetUserDataHandler(writer http.ResponseWriter, request *http.Request) {
   log.Println("GetUserDataHandler Serving:", request.URL.Path, "from", request.Host)
   id, ok := mux.Vars(request)["id"]
   if !ok {
      log.Println("ID value not set!")
      writer.WriteHeader(http.StatusBadRequest)
      return
   }

   intId, err := strconv.Atoi(id)
   if err != nil {
      log.Println("id", err)
      writer.WriteHeader(http.StatusBadRequest)
      return
   }

   userId := FindUserId(intId)
   if userId.Id != 0 {
      err := userId.ToJson(writer)
      if err != nil {
         writer.WriteHeader(http.StatusBadRequest)
         log.Println(err)
      }

      return
   }

   log.Println("User not found:", id)
   writer.WriteHeader(http.StatusBadRequest)
}
