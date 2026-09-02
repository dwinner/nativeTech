package main

// Database: Pg-SQL
//
// Functions to support the interaction with the database

import (
   "database/sql"
   "encoding/json"
   "fmt"
   "io"
   "log"
   "os"

   _ "github.com/lib/pq"
)

const PgConnStr = "user=postgres password=3680251am dbname=users sslmode=disable"

// OpenConnection Opens Pg-SQL connection to be used elsewhere
func OpenConnection() *sql.DB {
   db, err := sql.Open("postgres", PgConnStr)
   if err != nil {
      fmt.Println("Error connecting:", err)
      return nil
   }

   return db
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
func DeleteUser(anId int) bool {
   db := OpenConnection()
   if db == nil {
      log.Println("Cannot connect to Pg-SQL!")
      return false
   }

   defer func(aDb *sql.DB) {
      err := aDb.Close()
      if err != nil {
         log.Println("Error closing Pg-SQL")
         os.Exit(-1)
      }
   }(db)

   // Check if the user ID exists
   user := FindUserId(anId)
   if user.Id == 0 {
      log.Println("User", anId, "does not exist.")
      return false
   }

   stmt, err := db.Prepare("DELETE FROM users WHERE userId = $1")
   if err != nil {
      log.Println("DeleteUser:", err)
      return false
   }

   defer func(stmt *sql.Stmt) {
      err := stmt.Close()
      if err != nil {
         log.Println("Clearing statement failed")
         os.Exit(-1)
      }
   }(stmt)

   _, err = stmt.Exec(anId)
   if err != nil {
      log.Println("DeleteUser:", err)
      return false
   }

   return true
}

// InsertUser is for adding a new user to the database
func InsertUser(aUser User) bool {
   db := OpenConnection()
   if db == nil {
      fmt.Println("Cannot connect to Pg-SQL!")
      return false
   }

   defer func(aDb *sql.DB) {
      err := aDb.Close()
      if err != nil {
         fmt.Println("Cannot close Pg-SQL! ", err)
         os.Exit(-1)
      }
   }(db)

   if IsUserValid(aUser) {
      log.Println("User", aUser.Username, "already exists!")
      return false
   }

   stmt, err := db.Prepare("INSERT INTO users(username, password, last_login, admin, active) values($1,$2,$3,$4,$5)")
   if err != nil {
      log.Println("Adduser:", err)
      return false
   }

   defer func(aStatement *sql.Stmt) {
      err := aStatement.Close()
      if err != nil {
         fmt.Println("Cannot close the statement ", err)
         os.Exit(-1)
      }
   }(stmt)

   _, err = stmt.Exec(aUser.Username, aUser.Password, aUser.LastLogin, aUser.Admin, aUser.Active)
   if err != nil {
      return false
   }

   return true
}

// ListAllUsers is for returning all users from the database table
func ListAllUsers() []User {
   db := OpenConnection()
   if db == nil {
      fmt.Println("Cannot connect to Pg-SQL!")
      return []User{}
   }

   defer func(aDb *sql.DB) {
      _ = aDb.Close()
   }(db)

   rows, err := db.Query("SELECT * FROM users \n")
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
   db := OpenConnection()
   if db == nil {
      fmt.Println("Cannot connect to Pg-SQL!")
      return User{}
   }

   defer func(aDb *sql.DB) {
      _ = aDb.Close()
   }(db)

   row := db.QueryRow("SELECT * FROM users where userId = $1 \n", aUserId)
   user := User{}
   err := row.Scan(&user.Id, &user.Username, &user.Password, &user.LastLogin, &user.Admin, &user.Active)
   if err != nil {
      log.Println("Query:", err)
      return User{}
   }

   return user
}

// FindUsername is for returning a user record defined by a username
func FindUsername(aUsername string) User {
   db := OpenConnection()
   if db == nil {
      fmt.Println("Cannot connect to Pg-SQL!")
      return User{}
   }

   defer func(aDb *sql.DB) {
      _ = aDb.Close()
   }(db)

   rows, err := db.Query("SELECT * FROM users where username = $1 \n", aUsername)
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
         isAdmin, isActive}
      log.Println("Found user:", user)
   }

   return user
}

// ReturnLoggedUsers is for returning all logged-in users
func ReturnLoggedUsers() []User {
   db := OpenConnection()
   if db == nil {
      fmt.Println("Cannot connect to Pg-SQL!")
      return []User{}
   }

   defer func(db *sql.DB) {
      _ = db.Close()
   }(db)

   rows, err := db.Query("SELECT * FROM users WHERE active = 1 \n")
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
   db := OpenConnection()
   if db == nil {
      fmt.Println("Cannot connect to Pg-SQL!")
      return false
   }

   defer func(aDb *sql.DB) {
      _ = aDb.Close()
   }(db)

   statement := fmt.Sprintf(`SELECT * FROM users WHERE username = '%s'`, aUser.Username)
   row := db.QueryRow(statement)
   user := User{}
   err := row.Scan(&user.Id, &user.Username, &user.Password, &user.LastLogin, &user.Admin, &user.Active)
   if err != nil {
      log.Println("IsUserAdmin:", err)
      return false
   }

   if aUser.Username == user.Username && aUser.Password == user.Password && user.Admin == 1 {
      return true
   }

   return false
}

func IsUserValid(aUser User) bool {
   db := OpenConnection()
   if db == nil {
      fmt.Println("Cannot connect to Pg-SQL!")
      return false
   }

   defer func(aDb *sql.DB) {
      _ = aDb.Close()
   }(db)

   row := db.QueryRow("SELECT * FROM users WHERE username = $1 \n", aUser.Username)
   user := User{}
   err := row.Scan(&user.Id, &user.Username, &user.Password, &user.LastLogin, &user.Admin, &user.Active)
   if err != nil {
      log.Println(err)
      return false
   }

   if aUser.Username == user.Username && aUser.Password == user.Password {
      return true
   }

   return false
}

// UpdateUser allows you to update username
func UpdateUser(aUser User) bool {
   log.Println("Updating user:", aUser)
   db := OpenConnection()
   if db == nil {
      fmt.Println("Cannot connect to Pg-SQL!")
      return false
   }

   defer func(db *sql.DB) {
      _ = db.Close()
   }(db)

   stmt, err := db.Prepare("UPDATE users SET username=$1, password=$2, admin=$3, active=$4 WHERE userId = $5")
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
