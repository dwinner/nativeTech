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

// getIdCmd represents the getid command
var getIdCmd = &cobra.Command{
   Use:   "getid",
   Short: "Returns User ID, given a username",
   Long: `This command returns the User ID of a user, given
	their username.`,
   Run: func(command *cobra.Command, args []string) {
      endpoint := "/getid"
      user := User{Username: username, Password: password}

      // Convert data string to User Structure
      var user2 User
      err := json.Unmarshal([]byte(data), &user2)
      if err != nil {
         fmt.Println("Unmarshal:", err)
         os.Exit(1)
      }

      if user2.Username == "" {
         fmt.Println("Empty username!")
         return
      }

      // bytes.Buffer is both a Reader and a Writer
      buffer := new(bytes.Buffer)
      err = user.ToJson(buffer)
      if err != nil {
         fmt.Println("JSON:", err)
         return
      }

      URL := Server + Port + endpoint + "/" + user2.Username
      request, err := http.NewRequest(http.MethodGet, URL, buffer)
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

      if response.StatusCode != http.StatusFound {
         fmt.Println(response)
         return
      }

      var returnedUser = User{}
      _ = SliceFromJson(&returnedUser, response.Body)
      fmt.Println("User", returnedUser.Username, "has ID:", returnedUser.ID)
   },
}

func init() {
   rootCmd.AddCommand(getIdCmd)
}
