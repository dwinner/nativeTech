package cmd

import (
   "fmt"

   "github.com/spf13/cobra"
)

// searchCmd represents the search command
var searchCmd = &cobra.Command{
   Use:   "search",
   Short: "search command",
   Long:  `The search command search for information.`,
   Run: func(aCommand *cobra.Command, args []string) {
      logger.Info("Searching for:")
      logger.Info(sid)
      for idx, entry := range data {
         if entry.Filename == sid {
            fmt.Println(data[idx])
            break
         }
      }
   },
}

var sid string

func init() {
   rootCmd.AddCommand(searchCmd)
   searchCmd.Flags().StringVarP(&sid, "sid", "s", "", "Search Key")
   _ = searchCmd.MarkFlagRequired("sid")
}
