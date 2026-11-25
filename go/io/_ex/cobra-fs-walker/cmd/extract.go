package cmd

import (
   "cobra_fs_walker/fs_utils"
   "fmt"
   "os"

   "github.com/spf13/cobra"
)

var extractCommand = &cobra.Command{
   Use:     "extract",
   Aliases: []string{"e"},
   Short:   "Extract a file",
   Long:    "Extract a file from a given directory",
   Run: func(aCommand *cobra.Command, args []string) {
      rootDir, err := aCommand.Flags().GetString(dirFlag)
      if err != nil {
         rootDir = "."
      }

      fmt.Println("Root dir is ", rootDir)
      filePath, err := aCommand.Flags().GetString(fileFlag)
      if err != nil {
         fmt.Println("Error reading file name flag", err)
         return
      }

      if len(filePath) == 0 {
         fmt.Println("File name is empty")
      }

      fileSystem := os.DirFS(rootDir)
      bytesExtracted, err := fs_utils.Extract(fileSystem, filePath)
      if err != nil {
         fmt.Println(err)
         return
      }

      err = writeToFile(bytesExtracted, "temp.txt")
      if err != nil {
         fmt.Println(err)
      }
   },
}

func init() {
   rootCmd.AddCommand(extractCommand)
   extractCommand.Flags().StringP(dirFlag, "d", ".", "Root directory")
   extractCommand.Flags().StringP(fileFlag, "f", "", "File name to search")
}

func writeToFile(bytesToWrite []byte, aPath string) error {
   fd, err := os.OpenFile(aPath, os.O_CREATE|os.O_WRONLY, 0644)
   if err != nil {
      return err
   }

   defer func(fd *os.File) {
      err := fd.Close()
      if err != nil {
         fmt.Println(err)
         os.Exit(-1)
      }
   }(fd)

   writtenLen, err := fd.Write(bytesToWrite)
   if err != nil {
      return err
   }

   fmt.Printf("wrote %d bytes\n", writtenLen)
   return nil
}
