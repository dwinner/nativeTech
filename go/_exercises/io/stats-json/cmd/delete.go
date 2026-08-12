package cmd

import (
   "fmt"
   "log/slog"
   "os"
   "slices"

   "github.com/spf13/cobra"
)

// deleteCmd represents the delete command
var deleteCmd = &cobra.Command{
   Use:   "delete",
   Short: "Delete command",
   Long:  `A command for deleting data.`,
   Run: func(aCommand *cobra.Command, args []string) {
      logger = slog.New(slog.NewJSONHandler(os.Stderr, nil))
      slog.SetDefault(logger)
      _, ok := index[key]
      if ok {
         logger.Info("Found key:", key)
         fmt.Println("Found key:", key)
         delete(index, key)
      } else {
         fmtString := fmt.Sprintf("%s not found!", key)
         logger.Info(fmtString)
         return
      }

      // Now, delete it from data
      fmt.Println(data)
      for idx, entry := range data {
         if entry.Filename == key {
            data = slices.Delete(data, idx, idx+1)
            break
         }
      }

      err := saveJsonFile(Jsonfile)
      if err != nil {
         logger.Warn("Error saving data:", err)
      }

      fmtString := fmt.Sprintf("Deleting key %s:", key)
      logger.Info(fmtString)
   },
}

var key string

func init() {
   rootCmd.AddCommand(deleteCmd)
   deleteCmd.Flags().StringVarP(&key, "key", "k", "", "Key to delete")
   _ = deleteCmd.MarkFlagRequired("key")
}
