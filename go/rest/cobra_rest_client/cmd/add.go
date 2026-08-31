package cmd

import (
   "bytes"
   "encoding/json"
   "fmt"
   "net/http"
   "os"
   "time"

   "github.com/spf13/cobra"
)

var addCmd = &cobra.Command{
   Use:   "add",
   Short: "Add a new user",
   Long:  `Add a new user to the system.`,
   Run: func(command *cobra.Command, args []string) {
      endpoint := "/add"
      user1 := User{Username: username, Password: password}

      // Convert data string to User Structure
      var user2 User
      err := json.Unmarshal([]byte(data), &user2)
      if err != nil {
         fmt.Println("Unmarshal:", err)
         os.Exit(1)
      }

      var users []User
      users = append(users, user1)
      users = append(users, user2)

      // bytes.Buffer is both a Reader and a Writer
      bytesBuffer := new(bytes.Buffer)
      err = SliceToJson(users, bytesBuffer)
      if err != nil {
         fmt.Println("JSON:", err)
         return
      }

      request, err := http.NewRequest(http.MethodPost, Server+Port+endpoint, bytesBuffer)
      if err != nil {
         fmt.Println("GetAll – Error in req: ", err)
         return
      }

      request.Header.Set("Content-Type", "application/json")
      httpClient := &http.Client{
         Timeout: 15 * time.Second,
      }

      response, err := httpClient.Do(request)
      if err != nil {
         fmt.Println("Do:", err)
         return
      }

      if response.StatusCode != http.StatusOK {
         fmt.Println("Status code:", response.Status)
         return
      }

      fmt.Println("User", user2.Username, "added.")
   },
}

func init() {
   rootCmd.AddCommand(addCmd)
}
