package main

import (
   "encoding/json"
   "fmt"
)

// NoEmpty Ignoring empty fields in JSON
type NoEmpty struct {
   Name    string `json:"username"`
   Surname string `json:"surname"`
   Year    int    `json:"creation-year,omitempty"`
}

// Password Removing private fields and ignoring empty fields
type Password struct {
   Name    string `json:"username"`
   Surname string `json:"surname,omitempty"`
   Year    int    `json:"creation-year,omitempty"`
   Pass    string `json:"-"`
}

func main() {
   noEmpty := NoEmpty{Name: "Mihalis"}
   password := Password{Name: "Mihalis", Pass: "myPassword"}

   // Ignoring empty fields in JSON
   noEmptyVar, err := json.Marshal(&noEmpty)
   if err != nil {
      fmt.Println(err)
   } else {
      fmt.Printf("noEmptyVar decoded with value %s\n", noEmptyVar)
   }

   // Removing private fields
   passwordVar, err := json.Marshal(&password)
   if err != nil {
      fmt.Println(err)
   } else {
      fmt.Printf("password decoded with value %s\n", passwordVar)
   }
}
