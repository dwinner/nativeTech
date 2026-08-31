package cmd

import (
   "fmt"
   "io"
   "net/http"
   "os"
   "time"

   "github.com/spf13/cobra"
)

var timeCmd = &cobra.Command{
   Use:   "time",
   Short: "Get the time from the RESTful server",
   Long:  `This command mainly exists for making sure that the server works.`,
   Run: func(command *cobra.Command, args []string) {
      request, err := http.NewRequest("GET", Server+Port+"/time", nil)
      if err != nil {
         fmt.Println("Timefunction – Error in req: ", err)
         os.Exit(1)
      }

      httpClient := &http.Client{
         Timeout: 15 * time.Second,
      }

      response, err := httpClient.Do(request)
      if err != nil {
         fmt.Println(err)
         os.Exit(1)
      }

      if response == nil || (response.StatusCode == http.StatusNotFound) {
         fmt.Println(response)
         return
      }

      defer func(Body io.ReadCloser) {
         _ = Body.Close()
      }(response.Body)

      data, _ := io.ReadAll(response.Body)
      fmt.Print(string(data))
   },
}

func init() {
   rootCmd.AddCommand(timeCmd)
}
