package cmd

import (
   "encoding/json"
   "io"
   "log/slog"
   "os"

   "github.com/spf13/cobra"
)

type Entry struct {
   Filename string  `json:"filename"`
   Len      int     `json:"length"`
   Minimum  float64 `json:"minimum"`
   Maximum  float64 `json:"maximum"`
   Mean     float64 `json:"mean"`
   StdDev   float64 `json:"stddev"`
}

var logger *slog.Logger

// Jsonfile resides in the current directory
var Jsonfile = "./data.json"

type EntryCollection []Entry

var data = EntryCollection{}
var index map[string]int

// DeSerialize decodes a serialized slice with JSON records
func DeSerialize(aSlice any, aReader io.Reader) error {
   decoder := json.NewDecoder(aReader)
   return decoder.Decode(aSlice)
}

// Serialize serializes a slice with JSON records
func Serialize(aSlice any, aWriter io.Writer) error {
   encoder := json.NewEncoder(aWriter)
   return encoder.Encode(aSlice)
}

func saveJsonFile(aFilepath string) error {
   createdFile, err := os.Create(aFilepath)
   if err != nil {
      return err
   }

   defer createdFile.Close()
   err = Serialize(&data, createdFile)

   return err
}

func readJsonFile(aFilepath string) error {
   _, err := os.Stat(aFilepath)
   if err != nil {
      return err
   }

   openedFile, err := os.Open(aFilepath)
   if err != nil {
      return err
   }

   defer openedFile.Close()

   err = DeSerialize(&data, openedFile)
   if err != nil {
      return err
   }

   return nil
}

func createIndex() {
   index = make(map[string]int)
   for idx, entry := range data {
      key := entry.Filename
      index[key] = idx
   }
}

// rootCmd represents the base command when called without any subcommands
var rootCmd = &cobra.Command{
   Use:   "stats",
   Short: "Statistics application",
   Long:  `The statistics application.`,
   Run: func(aCommand *cobra.Command, args []string) {
   },
}

func Execute() {
   err := rootCmd.Execute()
   if err != nil {
      os.Exit(1)
   }

   logger = slog.New(slog.NewJSONHandler(os.Stderr, nil))

   // Work with logger
   if disableLogging == false {
      logger = slog.New(slog.NewJSONHandler(io.Discard, nil))
   }

   slog.SetDefault(logger)
}

var disableLogging bool

func init() {
   rootCmd.PersistentFlags().BoolVarP(&disableLogging, "log", "l", false, "Logging information")
   err := readJsonFile(Jsonfile)

   // io.EOF is fine because it means the file is empty
   if err != nil && err != io.EOF {
      return
   }

   createIndex()
}
