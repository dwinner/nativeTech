package cmd

import (
   "fmt"
   "wcutils"

   "github.com/spf13/cobra"
)

var linesCmd = &cobra.Command{
   Use:     "lines",
   Aliases: []string{"l"},
   Short:   "Print the line count",
   Long:    `Print the line count`,
   Run: func(aCommand *cobra.Command, args []string) {
      if args == nil || len(args) == 0 {
         return
      }

      for _, file := range args {
         lineCount, err := wcutils.CountByLine(file)
         if err == nil {
            fmt.Printf("The count of lines in file '%s': '%d'\n", file, lineCount)
         }
      }
   },
}

func init() {
   rootCmd.AddCommand(linesCmd)
}
