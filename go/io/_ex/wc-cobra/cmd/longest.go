package cmd

import (
   "fmt"
   "wcutils"

   "github.com/spf13/cobra"
)

var longestCmd = &cobra.Command{
   Use:     "longest",
   Aliases: []string{"L"},
   Short:   "Print the length of the longest string",
   Long:    "Print the length of the longest string",
   Run: func(aCommand *cobra.Command, args []string) {
      if args == nil || len(args) == 0 {
         return
      }

      maxLen := wcutils.ComputeLongestStringLen(args...)
      fmt.Printf("The longest string length is '%d'\n", maxLen)
   },
}

func init() {
   rootCmd.AddCommand(longestCmd)
}
