package cmd

import (
   "bytes"
   "encoding/json"
   "fmt"
   "io"
   "log/slog"
   "os"
   "sort"

   "github.com/spf13/cobra"
)

// listCmd represents the list command
var listCmd = &cobra.Command{
   Use:   "list",
   Short: "A brief description of your command",
   Long: `A longer description that spans multiple lines and likely contains examples
and usage of using your command. For example:

Cobra is a CLI library for Go that empowers applications.
This application is a tool to generate the needed files
to quickly create a Cobra application.`,
   Run: func(aCommand *cobra.Command, args []string) {
      list()
   },
}

func init() {
   rootCmd.AddCommand(listCmd)
}

func list() {
   sort.Sort(data)
   text, err := PrettyPrintJsonStream(data)
   if err != nil {
      fmt.Println(err)
      return
   }

   fmt.Println(text)
   logger = slog.New(slog.NewJSONHandler(os.Stderr, nil))

   // Work with logger
   if disableLogging == false {
      logger = slog.New(slog.NewJSONHandler(io.Discard, nil))
   }

   slog.SetDefault(logger)
   fmtStr := fmt.Sprintf("%d records in total.", len(data))
   logger.Info(fmtStr)
}

// PrettyPrintJsonStream pretty prints the contents of the phone book
func PrettyPrintJsonStream(aData any) (string, error) {
   buffer := new(bytes.Buffer)
   encoder := json.NewEncoder(buffer)
   encoder.SetIndent("", "\t")
   err := encoder.Encode(aData)
   if err != nil {
      return "", err
   }

   return buffer.String(), nil
}

// Len - Implement sort.Interface
func (anEntryColl EntryCollection) Len() int {
   return len(anEntryColl)
}

func (anEntryColl EntryCollection) Less(i, j int) bool {
   if anEntryColl[i].Mean == anEntryColl[j].Mean {
      return anEntryColl[i].StdDev < anEntryColl[j].StdDev
   }

   return anEntryColl[i].Mean < anEntryColl[j].Mean
}

func (anEntryColl EntryCollection) Swap(i, j int) {
   anEntryColl[i], anEntryColl[j] = anEntryColl[j], anEntryColl[i]
}
