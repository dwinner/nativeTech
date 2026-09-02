package main

import (
   "encoding/json"
   "fmt"
   "io"
   "log"
   "net/http"
)

// GetAllHandler is for getting all data from the user database
func GetAllHandler(writer http.ResponseWriter, request *http.Request) {
   log.Println("GetAllHandler Serving:", request.URL.Path, "from", request.Host)
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

   fmt.Println("GetAllHandler:", string(bytes))
   user := User{}
   err = json.Unmarshal(bytes, &user)
   if err != nil {
      log.Println("GetAllHandler:", err)
      writer.WriteHeader(http.StatusBadRequest)
      return
   }

   if !IsUserAdmin(user) {
      log.Println("(GetAllHandler:) User", user, "is not an admin!")
      writer.WriteHeader(http.StatusBadRequest)
      return
   }

   users := ListAllUsers()
   err = SliceToJson(users, writer)
   if err != nil {
      log.Println(err)
      writer.WriteHeader(http.StatusBadRequest)
      return
   }
}
