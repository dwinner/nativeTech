package main

import (
   "bytes"
   "encoding/json"
   "fmt"
   "math/rand"
)

type Data struct {
   Key string `json:"key"`
   Val int    `json:"value"`
}

var DataRecords []Data

func random(min, max int) int {
   return rand.Intn(max-min) + min
}

var MIN = 0
var MAX = 26

func getString(aLen int64) string {
   startChar := "A"
   temp := ""
   var i int64 = 1
   for {
      myRand := random(MIN, MAX)
      newChar := string(startChar[0] + byte(myRand))
      temp = temp + newChar
      if i == aLen {
         break
      }

      i++
   }

   return temp
}

// DeSerialize decodes a serialized slice with JSON records
func DeSerialize(aDecoder *json.Decoder, aSlice any) error {
   return aDecoder.Decode(aSlice)
}

// Serialize serializes a slice with JSON records
func Serialize(anEncoder *json.Encoder, aSlice any) error {
   return anEncoder.Encode(aSlice)
}

func main() {
   // Create sample data
   var data Data
   for range 2 {
      data = Data{
         Key: getString(5),
         Val: random(1, 100),
      }
      DataRecords = append(DataRecords, data)
   }

   // bytes.Buffer is both an io.Reader and io.Writer
   buf := new(bytes.Buffer)
   encoder := json.NewEncoder(buf)
   err := Serialize(encoder, DataRecords)
   if err != nil {
      fmt.Println(err)
      return
   }

   fmt.Print("After Serialize:", buf)
   decoder := json.NewDecoder(buf)
   var temp []Data
   err = DeSerialize(decoder, &temp)
   fmt.Println("After DeSerialize:")
   for index, value := range temp {
      fmt.Println(index, value)
   }
}
