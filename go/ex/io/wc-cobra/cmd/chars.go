package cmd

import (
   "fmt"
   "wcutils"

   "github.com/spf13/cobra"
)

var charsCmd = &cobra.Command{
   Use:     "chars",
   Aliases: []string{"c"},
   Short:   "Print the char count",
   Long:    "Print the char count",
   Run: func(aCommand *cobra.Command, args []string) {
      if args == nil || len(args) == 0 {
         return
      }

      for _, file := range args {
         charCount, err := wcutils.CountByChar(file)
         if err == nil {
            fmt.Printf("The count of chars in file '%s': '%d'\n", file, charCount)
         }
      }
   },
}

func init() {
   rootCmd.AddCommand(charsCmd)
}
