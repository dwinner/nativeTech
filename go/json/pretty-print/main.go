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

func getString(aLength int64) string {
   startChar := "A"
   temp := ""
   var i int64 = 1
   for {
      myRand := random(MIN, MAX)
      newChar := string(startChar[0] + byte(myRand))
      temp = temp + newChar
      if i == aLength {
         break
      }

      i++
   }

   return temp
}

func PrettyPrint(iface any) (err error) {
   bytesWritten, err := json.MarshalIndent(iface, "", "\t")
   if err == nil {
      fmt.Println(string(bytesWritten))
   }

   return err
}

func JsonStream(iface any) (string, error) {
   buffer := new(bytes.Buffer)
   encoder := json.NewEncoder(buffer)
   encoder.SetIndent("", "\t")
   err := encoder.Encode(iface)
   if err != nil {
      return "", err
   }

   return buffer.String(), nil
}

func main() {
   // Create random records
   var data Data
   for range 2 {
      data = Data{
         Key: getString(5),
         Val: random(1, 100),
      }
      DataRecords = append(DataRecords, data)
   }

   fmt.Println("Last record:", data)
   _ = PrettyPrint(data)

   val, _ := JsonStream(DataRecords)
   fmt.Println(val)
}
