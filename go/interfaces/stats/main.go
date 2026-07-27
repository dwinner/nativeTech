package main

import (
   "encoding/csv"
   "fmt"
   "math"
   "os"
   "slices"
   "sort"
   "strconv"
)

func readFile(aFilepath string) ([]float64, error) {
   _, err := os.Stat(aFilepath)
   if err != nil {
      return nil, err
   }

   file, err := os.Open(aFilepath)
   if err != nil {
      return nil, err
   }

   defer file.Close()
   lines, err := csv.NewReader(file).ReadAll()
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

func stdDev(anArray []float64) (float64, float64) {
   sum := float64(0)
   for _, val := range anArray {
      sum = sum + val
   }

   meanValue := sum / float64(len(anArray))
   fmt.Printf("Mean value: %.5f\n", meanValue)

   // Standard deviation
   var squared float64
   for i := 0; i < len(anArray); i++ {
      squared = squared + math.Pow(anArray[i]-meanValue, 2)
   }

   standardDeviation := math.Sqrt(squared / float64(len(anArray)))
   return meanValue, standardDeviation
}

type DataFile struct {
   Filename string
   Len      int
   Minimum  float64
   Maximum  float64
   Mean     float64
   StdDev   float64
}

type DataSlice []DataFile

// Len Implement sort.Interface
func (aDataSlice DataSlice) Len() int {
   return len(aDataSlice)
}

func (aDataSlice DataSlice) Less(i, j int) bool {
   return aDataSlice[i].Mean < aDataSlice[j].Mean
}

func (aDataSlice DataSlice) Swap(i, j int) {
   aDataSlice[i], aDataSlice[j] = aDataSlice[j], aDataSlice[i]
}

func main() {
   if len(os.Args) == 1 {
      fmt.Println("Need one or more file paths!")
      return
   }

   // Slice of DataFile structures
   files := DataSlice{}

   for i := 1; i < len(os.Args); i++ {
      file := os.Args[i]
      currentFile := DataFile{}
      currentFile.Filename = file
      values, err := readFile(file)
      if err != nil {
         fmt.Println("Error reading:", file, err)
         os.Exit(0)
      }

      currentFile.Len = len(values)
      currentFile.Minimum = slices.Min(values)
      currentFile.Maximum = slices.Max(values)
      meanValue, standardDeviation := stdDev(values)
      currentFile.Mean = meanValue
      currentFile.StdDev = standardDeviation

      files = append(files, currentFile)
   }

   // Now sort files
   sort.Sort(files)
   for _, val := range files {
      filename := val.Filename
      fmt.Println(filename, ":", val.Len, val.Mean, val.Maximum, val.Minimum)
   }
}
