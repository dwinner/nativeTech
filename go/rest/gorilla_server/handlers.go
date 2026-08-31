package main

import (
   "encoding/json"
   "fmt"
   "io"
   "log"
   "net/http"
   "strconv"
   "time"

   "github.com/gorilla/mux"
)

type User struct {
   Id        int    `json:"id"`
   Username  string `json:"username"`
   Password  string `json:"password"`
   LastLogin int64  `json:"lastlogin"`
   Admin     int    `json:"admin"`
   Active    int    `json:"active"`
}

// SliceToJson encodes a slice with JSON records
func SliceToJson(aSlice any, aWriter io.Writer) error {
   encoder := json.NewEncoder(aWriter)
   return encoder.Encode(aSlice)
}

type notAllowedHandler struct{}

func (aHandler notAllowedHandler) ServeHTTP(responseWriter http.ResponseWriter, request *http.Request) {
   MethodNotAllowedHandler(responseWriter, request)
}

func DefaultHandler(responseWriter http.ResponseWriter, request *http.Request) {
   log.Println("DefaultHandler Serving:", request.URL.Path, "from", request.Host, "with method", request.Method)
   responseWriter.WriteHeader(http.StatusNotFound)
   body := request.URL.Path + " is not supported. Thanks for visiting!\n"
   _, _ = fmt.Fprintf(responseWriter, "%s", body)
}

// MethodNotAllowedHandler is executed when the HTTP method is incorrect
func MethodNotAllowedHandler(responseWriter http.ResponseWriter, request *http.Request) {
   log.Println("Serving:", request.URL.Path, "from", request.Host, "with method", request.Method)
   responseWriter.WriteHeader(http.StatusNotFound)
   body := "Method not allowed!\n"
   _, _ = fmt.Fprintf(responseWriter, "%s", body)
}

// TimeHandler is for handling /time – it works with plain text
func TimeHandler(responseWriter http.ResponseWriter, request *http.Request) {
   log.Println("TimeHandler Serving:", request.URL.Path, "from", request.Host)
   responseWriter.WriteHeader(http.StatusOK)
   fmtTime := time.Now().Format(time.RFC1123)
   Body := "The current time is: " + fmtTime + "\n"
   _, _ = fmt.Fprintf(responseWriter, "%s", Body)
}

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

   err = SliceToJson(ListAllUsers(), writer)
   if err != nil {
      log.Println(err)
      writer.WriteHeader(http.StatusBadRequest)
      return
   }
}

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
