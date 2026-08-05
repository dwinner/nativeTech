package cmd

import (
   "fmt"
   "wcutils"

   "github.com/spf13/cobra"
)

var wordsCmd = &cobra.Command{
   Use:     "words",
   Aliases: []string{"w"},
   Short:   "Print the word count",
   Long:    "Print the word count",
   Run: func(aCommand *cobra.Command, args []string) {
      if args == nil || len(args) == 0 {
         return
      }

      for _, file := range args {
         wordCount, err := wcutils.CountByWord(file)
         if err == nil {
            fmt.Printf("The count of words in file '%s': '%d'\n", file, wordCount)
         }
      }
   },
}

func init() {
   rootCmd.AddCommand(wordsCmd)
}
