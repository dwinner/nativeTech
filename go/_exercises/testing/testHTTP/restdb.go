package main

// Database: SQLite3
//
// Functions to support the interaction with the database

import (
   "database/sql"
   "encoding/json"
   "fmt"
   "io"
   "log"

   _ "github.com/mattn/go-sqlite3"
)

var Filename = "REST.db"

// OpenConnection Opens SQLite3 connection to be used elsewhere
func OpenConnection() *sql.DB {
   dbConn, err := sql.Open("sqlite3", Filename)
   if err != nil {
      fmt.Println("Error connecting:", err)
      return nil
   }

   return dbConn
}

// FromJson decodes a serialized JSON record - User{}
func (aUser *User) FromJson(aReader io.Reader) error {
   decoder := json.NewDecoder(aReader)
   return decoder.Decode(aUser)
}

// ToJson encodes a User JSON record
func (aUser *User) ToJson(aWriter io.Writer) error {
   encoder := json.NewEncoder(aWriter)
   return encoder.Encode(aUser)
}

// DeleteUser is for deleting a user defined by ID
func DeleteUser(aUserId int) bool {
   connection := OpenConnection()
   if connection == nil {
      log.Println("Cannot connect to SQLite3!")
      return false
   }

   defer func(connection *sql.DB) {
      err := connection.Close()
      if err != nil {
         log.Fatal("Cannot close connection: ", err)
      }
   }(connection)

   // Check is the user ID exists
   user := FindUserId(aUserId)
   if user.Id == 0 {
      log.Println("User", aUserId, "does not exist.")
      return false
   }

   stmt, err := connection.Prepare("DELETE FROM users WHERE UserID = $1")
   if err != nil {
      log.Println("DeleteUser:", err)
      return false
   }

   defer func(stmt *sql.Stmt) {
      _ = stmt.Close()
   }(stmt)

   _, err = stmt.Exec(aUserId)
   if err != nil {
      log.Println("DeleteUser:", err)
      return false
   }

   return true
}

// InsertUser is for adding a new user to the database
func InsertUser(aUser User) bool {
   connection := OpenConnection()
   if connection == nil {
      fmt.Println("Cannot connect to SQLite3!")
      return false
   }

   defer func(connection *sql.DB) {
      err := connection.Close()
      if err != nil {
         log.Fatal("Error closing connection")
      }
   }(connection)

   if IsUserValid(aUser) {
      log.Println("User", aUser.Username, "already exists!")
      return false
   }

   stmt, err := connection.Prepare("INSERT INTO users(username, password, lastlogin, admin, active) values($1,$2,$3,$4,$5)")
   if err != nil {
      log.Println("Adduser:", err)
      return false
   }

   defer func(stmt *sql.Stmt) {
      _ = stmt.Close()
   }(stmt)

   _, _ = stmt.Exec(aUser.Username, aUser.Password, aUser.LastLogin, aUser.Admin, aUser.Active)
   return true
}

// ListAllUsers is for returning all users from the database table
func ListAllUsers() []User {
   connection := OpenConnection()
   if connection == nil {
      fmt.Println("Cannot connect to SQLite3!")
      return []User{}
   }

   defer func(connection *sql.DB) {
      err := connection.Close()
      if err != nil {
         log.Fatal("Error closing connection: ", err)
      }
   }(connection)

   rows, err := connection.Query("SELECT * FROM users \n")
   if err != nil {
      log.Println(err)
      return []User{}
   }

   defer func(rows *sql.Rows) {
      _ = rows.Close()
   }(rows)

   var all []User
   var userId int
   var userName, userPass string
   var lastLogin int64
   var isAdmin, isActive int

   for rows.Next() {
      err = rows.Scan(&userId, &userName, &userPass, &lastLogin, &isAdmin, &isActive)
      user := User{
         userId,
         userName,
         userPass,
         lastLogin,
         isAdmin,
         isActive,
      }

      all = append(all, user)
   }

   log.Println("All:", all)
   return all
}

// FindUserId is for returning a user record defined by ID
func FindUserId(aUserId int) User {
   connection := OpenConnection()
   if connection == nil {
      fmt.Println("Cannot connect to SQLite!")
      return User{}
   }

   defer func(connection *sql.DB) {
      err := connection.Close()
      if err != nil {
         log.Fatal("Error closing connection: ", err)
      }
   }(connection)

   rows, err := connection.Query("SELECT * FROM users where UserID = $1 \n", aUserId)
   if err != nil {
      log.Println("Query:", err)
      return User{}
   }

   defer func(rows *sql.Rows) {
      _ = rows.Close()
   }(rows)

   var user = User{}
   var userId int
   var userName, userPass string
   var lastLogin int64
   var isAdmin, isActive int

   for rows.Next() {
      err = rows.Scan(&userId, &userName, &userPass, &lastLogin, &isAdmin, &isActive)
      if err != nil {
         log.Println(err)
         return User{}
      }

      user = User{userId,
         userName,
         userPass,
         lastLogin,
         isAdmin,
         isActive}
      log.Println("Found user:", user)
   }

   return user
}

// FindUserName is for returning a user record defined by a username
func FindUserName(aUsername string) User {
   connection := OpenConnection()
   if connection == nil {
      fmt.Println("Cannot connect to SQLite3!")
      return User{}
   }

   defer func(connection *sql.DB) {
      err := connection.Close()
      if err != nil {
         log.Fatal("Error closing connection: ", err)
      }
   }(connection)

   rows, err := connection.Query("SELECT * FROM users where username = $1 \n", aUsername)
   if err != nil {
      log.Println("FindUserUsername Query:", err)
      return User{}
   }

   defer func(rows *sql.Rows) {
      _ = rows.Close()
   }(rows)

   user := User{}
   var userId int
   var userName, userPass string
   var lastLogin int64
   var isAdmin, isActive int

   for rows.Next() {
      err = rows.Scan(&userId, &userName, &userPass, &lastLogin, &isAdmin, &isActive)
      if err != nil {
         log.Println(err)
         return User{}
      }

      user = User{userId,
         userName,
         userPass,
         lastLogin,
         isAdmin,
         isActive}
      log.Println("Found user:", user)
   }

   return user
}

// ReturnLoggedUsers is for returning all logged in users
func ReturnLoggedUsers() []User {
   connection := OpenConnection()
   if connection == nil {
      fmt.Println("Cannot connect to SQLite3!")
      return []User{}
   }

   defer func(connection *sql.DB) {
      _ = connection.Close()
   }(connection)

   rows, err := connection.Query("SELECT * FROM users WHERE active = 1 \n")
   if err != nil {
      log.Println(err)
      return []User{}
   }

   defer func(rows *sql.Rows) {
      _ = rows.Close()
   }(rows)

   var all []User
   var userId int
   var userName, userPass string
   var lastLogin int64
   var isAdmin, isActive int

   for rows.Next() {
      err = rows.Scan(&userId, &userName, &userPass, &lastLogin, &isAdmin, &isActive)
      if err != nil {
         log.Println(err)
         return []User{}
      }

      user := User{userId,
         userName,
         userPass,
         lastLogin,
         isAdmin,
         isActive}
      log.Println("temp:", all)
      all = append(all, user)
   }

   log.Println("Logged in:", all)
   return all
}

// IsUserAdmin determines whether a user is
// an administrator or not
func IsUserAdmin(aUser User) bool {
   connection := OpenConnection()
   if connection == nil {
      fmt.Println("Cannot connect to SQLite3!")
      return false
   }

   defer func(connection *sql.DB) {
      _ = connection.Close()
   }(connection)

   statement := fmt.Sprintf(`SELECT * FROM users WHERE username = '%s'`, aUser.Username)
   rows, err := connection.Query(statement)
   if err != nil {
      log.Println("IsUserAdmin:", err)
      return false
   }

   defer func(rows *sql.Rows) {
      _ = rows.Close()
   }(rows)

   user := User{}
   var userId int
   var userName, userPass string
   var lastLogin int64
   var isAdmin, isActive int

   // If there exist multiple users with the same username,
   // we will get the FIRST ONE only.
   for rows.Next() {
      err = rows.Scan(&userId, &userName, &userPass, &lastLogin, &isAdmin, &isActive)
      if err != nil {
         log.Println("IsUserAdmin:", err)
         return false
      }

      user = User{userId,
         userName,
         userPass,
         lastLogin,
         isAdmin,
         isActive}
   }

   if aUser.Username == user.Username && aUser.Password == user.Password && user.Admin == 1 {
      return true
   }

   return false
}

func IsUserValid(aUser User) bool {
   connection := OpenConnection()
   if connection == nil {
      fmt.Println("Cannot connect to SQLite3!")
      return false
   }

   defer func(connection *sql.DB) {
      _ = connection.Close()
   }(connection)

   rows, err := connection.Query("SELECT * FROM users WHERE username = $1 \n", aUser.Username)
   if err != nil {
      log.Println(err)
      return false
   }

   defer func(rows *sql.Rows) {
      _ = rows.Close()
   }(rows)

   user := User{}
   var userId int
   var userName, userPass string
   var lastLogin int64
   var isAdmin, isActive int

   // If there exist multiple users with the same username,
   // we will get the FIRST ONE only.
   for rows.Next() {
      err = rows.Scan(&userId, &userName, &userPass, &lastLogin, &isAdmin, &isActive)
      if err != nil {
         log.Println(err)
         return false
      }

      user = User{userId,
         userName,
         userPass,
         lastLogin,
         isAdmin,
         isActive}
   }

   if aUser.Username == user.Username && aUser.Password == user.Password {
      return true
   }

   return false
}

// UpdateUser allows you to update username
func UpdateUser(aUser User) bool {
   log.Println("Updating user:", aUser)
   connection := OpenConnection()
   if connection == nil {
      fmt.Println("Cannot connect to SQLite3!")
      return false
   }

   defer func(connection *sql.DB) {
      _ = connection.Close()
   }(connection)

   stmt, err := connection.Prepare("UPDATE users SET username=$1, password=$2, admin=$3, active=$4 WHERE UserID = $5")
   if err != nil {
      log.Println("Adduser:", err)
      return false
   }

   defer func(stmt *sql.Stmt) {
      _ = stmt.Close()
   }(stmt)

   res, err := stmt.Exec(aUser.Username, aUser.Password, aUser.Admin, aUser.Active, aUser.Id)
   if err != nil {
      log.Println("UpdateUser failed:", err)
      return false
   }

   affect, err := res.RowsAffected()
   if err != nil {
      log.Println("RowsAffected() failed:", err)
      return false
   }

   log.Println("Affected:", affect)
   return true
}
