package main

import (
   "encoding/csv"
   "fmt"
   "log"
   "math"
   "os"
   "sort"
   "strconv"
)

func readFile(aFilepath string) ([]float64, error) {
   _, err := os.Stat(aFilepath)
   if err != nil {
      return nil, err
   }

   fDesc, err := os.Open(aFilepath)
   if err != nil {
      return nil, err
   }

   defer fDesc.Close()
   lines, err := csv.NewReader(fDesc).ReadAll()
   if err != nil {
      return nil, err
   }

   values := make([]float64, 0)
   for _, line := range lines {
      tmp, err := strconv.ParseFloat(line[0], 64)
      if err != nil {
         log.Println("Error reading:", line[0], err)
         continue
      }

      values = append(values, tmp)
   }

   return values, nil
}

func stdDev(anInputArr []float64) (float64, float64) {
   sum := 0.0
   for _, val := range anInputArr {
      sum = sum + val
   }

   meanValue := sum / float64(len(anInputArr))
   fmt.Printf("Mean value: %.5f\n", meanValue)

   // Standard deviation
   var squared float64
   for i := 0; i < len(anInputArr); i++ {
      squared = squared + math.Pow(anInputArr[i]-meanValue, 2)
   }

   stdDev := math.Sqrt(squared / float64(len(anInputArr)))
   return meanValue, stdDev
}

func normalize(data []float64, mean float64, stdDev float64) []float64 {
   if stdDev == 0 {
      return data
   }

   const total = 10000
   normalized := make([]float64, len(data))
   for i, val := range data {
      normalized[i] = math.Floor((val-mean)/stdDev*total) / total
   }

   return normalized
}

func main() {
   if len(os.Args) == 1 {
      log.Println("Need one argument!")
      return
   }

   file := os.Args[1]
   values, err := readFile(file)
   if err != nil {
      log.Println("Error reading:", file, err)
      os.Exit(0)
   }

   sort.Float64s(values)
   fmt.Println("Number of values:", len(values))
   fmt.Println("Min:", values[0])
   fmt.Println("Max:", values[len(values)-1])

   mean, stdDev := stdDev(values)
   fmt.Printf("Standard deviation: %.5f\n", stdDev)

   // Now normalize the list of values
   normalized := normalize(values, mean, stdDev)
   fmt.Println("Normalized:", normalized)
}
