package pdo

import (
   "encoding/json"
   "io"
   "math"
   "slices"
)

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

func Process(aFilename string, values []float64) Entry {
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
