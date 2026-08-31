package cmd

import (
   "bytes"
   "fmt"
   "net/http"
   "os"
   "time"

   "github.com/spf13/cobra"
)

var logoutCmd = &cobra.Command{
   Use:   "logout",
   Short: "Logout for user",
   Long:  `Logging out a user from the system.`,
   Run: func(command *cobra.Command, args []string) {
      endpoint := "/logout"
      user := User{Username: username, Password: password}

      // bytes.Buffer is both a Reader and a Writer
      buf := new(bytes.Buffer)
      err := user.ToJson(buf)
      if err != nil {
         fmt.Println(err)
         os.Exit(1)
      }

      request, err := http.NewRequest(http.MethodPost, Server+Port+endpoint, buf)
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

      fmt.Println("User", user.Username, "logged out!")
   },
}

func init() {
   rootCmd.AddCommand(logoutCmd)
}
