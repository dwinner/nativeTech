package cmd

import (
   "encoding/csv"
   "fmt"
   "io"
   "log/slog"
   "math"
   "os"
   "slices"
   "strconv"

   "github.com/spf13/cobra"
)

// insertCmd represents the insert command
var insertCmd = &cobra.Command{
   Use:   "insert",
   Short: "Insert command",
   Long: `The insert command reads a datafile and stores
	its data into the application in JSON format.`,
   Run: func(cmd *cobra.Command, args []string) {
      logger = slog.New(slog.NewJSONHandler(os.Stderr, nil))

      // Work with logger
      if disableLogging == false {
         logger = slog.New(slog.NewJSONHandler(io.Discard, nil))
      }

      slog.SetDefault(logger)
      if file == "" {
         logger.Info("Need a file to read!")
         return
      }

      _, ok := index[file]
      if ok {
         fmt.Println("Found key:", file)
         delete(index, file)
      }

      // Now, delete it from data
      if ok {
         for idx, entry := range data {
            if entry.Filename == file {
               data = slices.Delete(data, idx, idx+1)
               break
            }
         }
      }

      err := ProcessFile(file)
      if err != nil {
         fmtString := fmt.Sprintf("Error processing: %s", err)
         logger.Warn(fmtString)
      }

      err = saveJsonFile(Jsonfile)
      if err != nil {
         fmtString := fmt.Sprintf("Error saving data: %s", err)
         logger.Info(fmtString)
      }
   },
}

var file string

func init() {
   rootCmd.AddCommand(insertCmd)

   // define required local flag
   insertCmd.Flags().StringVarP(&file, "file", "f", "", "Filename to process")
   _ = insertCmd.MarkFlagRequired("file")
   logger = slog.New(slog.NewJSONHandler(os.Stderr, nil))

   // Work with logger
   if disableLogging == false {
      logger = slog.New(slog.NewJSONHandler(io.Discard, nil))
   }

   slog.SetDefault(logger)
   fmtString := fmt.Sprintf("%d records in total.", len(data))
   logger.Info(fmtString)
}

func readFile(aFilepath string) ([]float64, error) {
   _, err := os.Stat(aFilepath)
   if err != nil {
      return nil, err
   }

   fileDesc, err := os.Open(aFilepath)
   if err != nil {
      return nil, err
   }
   defer fileDesc.Close()
   lines, err := csv.NewReader(fileDesc).ReadAll()
   if err != nil {
      return nil, err
   }

   values := make([]float64, 0)
   for _, line := range lines {
      tmp, err := strconv.ParseFloat(line[0], 64)
      if err != nil {
         fmt.Println("Error reading:", line[0], err)
         continue
      }

      values = append(values, tmp)
   }

   return values, nil
}

func stdDev(floatSlice []float64) (float64, float64) {
   sum := float64(0)
   for _, val := range floatSlice {
      sum = sum + val
   }

   meanValue := sum / float64(len(floatSlice))

   // Standard deviation
   var squared float64
   for i := range floatSlice {
      squared = squared + math.Pow(floatSlice[i]-meanValue, 2)
   }

   standardDeviation := math.Sqrt(squared / float64(len(floatSlice)))
   return meanValue, standardDeviation
}

func ProcessFile(aFile string) error {
   currentFile := Entry{}
   currentFile.Filename = aFile

   values, err := readFile(aFile)
   if err != nil {
      return err
   }

   currentFile.Len = len(values)
   currentFile.Minimum = slices.Min(values)
   currentFile.Maximum = slices.Max(values)
   meanValue, standardDeviation := stdDev(values)
   currentFile.Mean = meanValue
   currentFile.StdDev = standardDeviation

   data = append(data, currentFile)

   return nil
}
