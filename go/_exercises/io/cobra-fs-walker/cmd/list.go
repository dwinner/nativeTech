package cmd

import (
   "cobra_fs_walker/fs_utils"
   "fmt"
   "os"

   "github.com/spf13/cobra"
)

const dirFlag = "directory"

var listCommand = &cobra.Command{
   Use:     "list",
   Aliases: []string{"l"},
   Short:   "List all files",
   Long:    "List all files in a given directory",
   Run: func(aCommand *cobra.Command, args []string) {
      rootDir, err := aCommand.Flags().GetString(dirFlag)
      if err != nil {
         rootDir = "."
      }

      fileSystem := os.DirFS(rootDir)
      err = fs_utils.List(fileSystem, rootDir)
      if err != nil {
         fmt.Println("Error occurred", err.Error())
      }
   },
}

func init() {
   rootCmd.AddCommand(listCommand)
   listCommand.Flags().StringP(dirFlag, "d", ".", "Root directory")
}
