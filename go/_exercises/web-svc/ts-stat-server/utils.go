package main

import (
   "encoding/json"
   "io"
   "math"
   "os"
   "slices"
)

func readJsonFile(aFilepath string) error {
   _, err := os.Stat(aFilepath)
   if err != nil {
      if os.IsNotExist(err) {
         return nil
      }

      return err
   }

   file, err := os.Open(aFilepath)
   if err != nil {
      return err
   }

   defer func(aFile *os.File) {
      _ = aFile.Close()
   }(file)

   err = DeSerialize(&phoneBook, file)
   if err != nil {
      return err
   }

   return nil
}

func createIndex() {
   index = make(map[string]int)
   for idx, entry := range phoneBook {
      key := entry.Name
      index[key] = idx
   }
}

func saveJsonFile(aFilepath string) error {
   file, err := os.Create(aFilepath)
   if err != nil {
      return err
   }

   defer func(file *os.File) {
      _ = file.Close()
   }(file)

   err = Serialize(&phoneBook, file)
   if err != nil {
      return err
   }

   return nil
}

// Serialize serializes a slice with JSON records
func Serialize(aSlice any, aWriter io.Writer) error {
   encoder := json.NewEncoder(aWriter)
   return encoder.Encode(aSlice)
}

// DeSerialize decodes a serialized slice with JSON records
func DeSerialize(aSlice any, aReader io.Reader) error {
   decoder := json.NewDecoder(aReader)
   return decoder.Decode(aSlice)
}

func process(aFilename string, values []float64) Entry {
   currentEntry := Entry{}
   currentEntry.Name = aFilename
   currentEntry.Len = len(values)
   currentEntry.Minimum = slices.Min(values)
   currentEntry.Maximum = slices.Max(values)
   meanValue, standardDeviation := stdDev(values)
   currentEntry.Mean = meanValue
   currentEntry.StdDev = standardDeviation

   return currentEntry
}

func stdDev(aFloatSlice []float64) (float64, float64) {
   sum := float64(0)
   for _, val := range aFloatSlice {
      sum = sum + val
   }

   meanValue := sum / float64(len(aFloatSlice))

   // Standard deviation
   var squared float64
   for idx := range aFloatSlice {
      squared = squared + math.Pow(aFloatSlice[idx]-meanValue, 2)
   }

   standardDeviation := math.Sqrt(squared / float64(len(aFloatSlice)))
   return meanValue, standardDeviation
}
