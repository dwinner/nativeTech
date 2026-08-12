package cmd

import (
   "fmt"
   "wcutils"

   "github.com/spf13/cobra"
)

var bytesCmd = &cobra.Command{
   Use:     "bytes",
   Aliases: []string{"b"},
   Short:   "Print the byte count",
   Long:    "Print the byte count",
   Run: func(aCommand *cobra.Command, args []string) {
      if args == nil || len(args) == 0 {
         return
      }

      for _, file := range args {
         byteCount, err := wcutils.CountByBytes(file)
         if err == nil {
            fmt.Printf("The count of bytes in file '%s': '%d'\n", file, byteCount)
         }
      }
   },
}

func init() {
   rootCmd.AddCommand(bytesCmd)
}
