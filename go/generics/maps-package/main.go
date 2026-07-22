package main

import (
   "fmt"
   "maps"
)

func main() {
   strIntMap := map[string]int{
      "one":   1,
      "two":   2,
      "three": 3,
      "four":  4,
   }

   maps.DeleteFunc(strIntMap, deleteCallback)
   fmt.Println(strIntMap)

   clonedMap := maps.Clone(strIntMap)
   if maps.Equal(strIntMap, clonedMap) {
      fmt.Println("Equal!")
   } else {
      fmt.Println("Not equal!")
   }

   clonedMap["three"] = 3
   clonedMap["two"] = 22

   fmt.Println("Before n:", clonedMap, "m:", strIntMap)
   maps.Copy(strIntMap, clonedMap)
   fmt.Println("After n:", clonedMap, "m:", strIntMap)

   strToIntMap := map[string]int{
      "one":   1,
      "two":   2,
      "three": 3,
      "four":  4,
   }

   floatMap := map[string]float64{
      "one":   1.00,
      "two":   2.00,
      "three": 3.00,
      "four":  4.00,
   }

   eq := maps.EqualFunc(strToIntMap, floatMap, equalCallback)
   fmt.Println("Is t equal to mFloat?", eq)
}

func deleteCallback(aKey string, aValue int) bool {
   return aValue%2 != 0
}

func equalCallback(aValue1 int, aValue2 float64) bool {
   return float64(aValue1) == aValue2
}
