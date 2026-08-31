package cmd

import (
   "bytes"
   "fmt"
   "net/http"
   "os"
   "time"

   "github.com/spf13/cobra"
)

// loggedCmd represents the logged command
var loggedCmd = &cobra.Command{
   Use:   "logged",
   Short: "List add logged in users",
   Long:  `This command shows all logged in users.`,
   Run: func(cmd *cobra.Command, args []string) {
      endpoint := "/logged"
      user := User{Username: username, Password: password}

      // bytes.Buffer is both a Reader and a Writer
      buffer := new(bytes.Buffer)
      err := user.ToJson(buffer)
      if err != nil {
         fmt.Println("JSON:", err)
         os.Exit(1)
      }

      request, err := http.NewRequest(http.MethodGet, Server+Port+endpoint, buffer)
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

      users := []User{}
      _ = SliceFromJson(&users, response.Body)
      data, err := PrettyJson(users)
      if err != nil {
         fmt.Println(err)
         return
      }

      fmt.Print(data)
   },
}

func init() {
   rootCmd.AddCommand(loggedCmd)
}
