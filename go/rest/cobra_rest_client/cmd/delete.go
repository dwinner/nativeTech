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

var deleteCmd = &cobra.Command{
   Use:   "delete",
   Short: "Deleting users",
   Long:  `This command deletes existing users from the database.`,
   Run: func(command *cobra.Command, args []string) {
      endpoint := "/username"
      user := User{Username: username, Password: password}

      // Convert data string to User Structure
      var user2 User
      err := json.Unmarshal([]byte(data), &user2)
      if err != nil {
         fmt.Println("Unmarshal:", err)
         os.Exit(1)
      }

      // bytes.Buffer is both a Reader and a Writer
      buffer := new(bytes.Buffer)
      err = user.ToJson(buffer)
      if err != nil {
         fmt.Println("JSON:", err)
         os.Exit(1)
      }

      URL := Server + Port + endpoint + "/" + fmt.Sprint(user2.ID)
      request, err := http.NewRequest(http.MethodDelete, URL, buffer)
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
      } else {
         fmt.Println("User with ID", user2.ID, "deleted.")
      }
   },
}

func init() {
   rootCmd.AddCommand(deleteCmd)
}
