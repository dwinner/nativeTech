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

var searchCmd = &cobra.Command{
   Use:   "search",
   Short: "Search the database",
   Long: `Search the database for a user, identified by a User ID.
	The command returns the full record of the user.`,
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
      buf := new(bytes.Buffer)
      err = user.ToJson(buf)
      if err != nil {
         fmt.Println("JSON:", err)
         return
      }

      URL := Server + Port + endpoint + "/" + fmt.Sprint(user2.ID)
      request, err := http.NewRequest(http.MethodGet, URL, buf)
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
         fmt.Println(response)
         return
      }

      var returned = User{}
      _ = SliceFromJson(&returned, response.Body)
      data, err := PrettyJson(returned)
      if err != nil {
         fmt.Println(err)
         return
      }

      fmt.Print(data)
   },
}

func init() {
   rootCmd.AddCommand(searchCmd)
}
