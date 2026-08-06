package cmd

import (
   "cobra_fs_walker/fs_utils"
   "fmt"
   "os"

   "github.com/spf13/cobra"
)

const fileFlag = "file"

var searchCommand = &cobra.Command{
   Use:     "search",
   Aliases: []string{"s"},
   Short:   "Search a file",
   Long:    "Search a file in a given directory",
   Run: func(aCommand *cobra.Command, args []string) {
      rootDir, err := aCommand.Flags().GetString(dirFlag)
      if err != nil {
         rootDir = "."
      }

      fmt.Println("Root dir is ", rootDir)
      fileName, err := aCommand.Flags().GetString(fileFlag)
      if err != nil {
         fmt.Println("Error reading file name flag", err)
         return
      }

      if len(fileName) == 0 {
         fmt.Println("File name is empty")
      }

      fileSystem := os.DirFS(rootDir)
      err = fs_utils.Search(fileSystem, rootDir, fileName)
      if err != nil {
         fmt.Println("Error ", err.Error())
      }
   },
}

func init() {
   rootCmd.AddCommand(searchCommand)
   searchCommand.Flags().StringP(dirFlag, "d", ".", "Root directory")
   searchCommand.Flags().StringP(fileFlag, "f", "", "File name to search")
}
