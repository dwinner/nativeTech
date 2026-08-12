package main

import (
   "fmt"
   "math"
   "math/rand"
   "os"
   "sort"
   "strconv"
)

func normalize(data []float64, mean float64, stdDev float64) []float64 {
   if stdDev == 0 {
      return data
   }

   const seed = 10000
   normalized := make([]float64, len(data))
   for i, val := range data {
      normalized[i] = math.Floor((val-mean)/stdDev*seed) / seed
   }

   return normalized
}

func randomFloat(min, max float64) float64 {
   return min + rand.Float64()*(max-min)
}

func main() {
   arguments := os.Args
   if len(arguments) == 1 {
      fmt.Println("Need one or more arguments!")
      return
   }

   rndNum, err := strconv.ParseInt(arguments[1], 10, 64)
   if err != nil || rndNum < 1 {
      fmt.Println("The 1st argument should be positive integer value")
      return
   }

   var values []float64
   for i := int64(0); i < rndNum; i++ {
      floatVal := randomFloat(-10, 10)
      values = append(values, floatVal)
   }

   sort.Float64s(values)

   fmt.Println("Number of values:", len(values))
   fmt.Println("Min:", values[0])
   fmt.Println("Max:", values[len(values)-1])

   sum := float64(0)
   for _, val := range values {
      sum = sum + val
   }

   meanValue := sum / float64(len(values))
   fmt.Printf("Mean value: %.5f\n", meanValue)

   // Standard deviation
   // https://en.wikipedia.org/wiki/Standard_deviation
   var squared float64
   for i := 0; i < len(values); i++ {
      squared = squared + math.Pow(values[i]-meanValue, 2)
   }

   stdDev := math.Sqrt(squared / float64(len(values)))
   fmt.Printf("Standard deviation: %.5f\n", stdDev)

   // Now normalize the list of values
   normalized := normalize(values, meanValue, stdDev)
   fmt.Println("Normalized:", normalized)
}
