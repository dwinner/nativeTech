package main

import (
   "encoding/json"
   "fmt"
   "os"
)

var JsonRecord = `{
	"Flag": true,
	"Array": ["a","b","c"],
	"Entity": {
	  "a1": "b1",
	  "a2": "b2",
	  "Value": -456,
	  "Null": null
	},
	"Message": "Hello Go!"
  }`

func typeSwitch(aMap map[string]interface{}) {
   for key, val := range aMap {
      switch typeDetect := val.(type) {
      case string:
         fmt.Println("Is a string!", key, typeDetect)
      case float64:
         fmt.Println("Is a float64!", key, typeDetect)
      case bool:
         fmt.Println("Is a Boolean!", key, typeDetect)
      case map[string]interface{}:
         fmt.Println("Is a map!", key, typeDetect)
         typeSwitch(val.(map[string]interface{}))
      default:
         fmt.Printf("...Is %v: %T!\n", key, typeDetect)
      }
   }

   return
}

func exploreMap(aMap map[string]interface{}) {
   for key, val := range aMap {
      embMap, ok := val.(map[string]interface{})

      // If it is a map, explore deeper
      if ok {
         fmt.Printf("{\"%v\": \n", key)
         exploreMap(embMap)
         fmt.Printf("}\n")
      } else {
         fmt.Printf("%v: %v\n", key, val)
      }
   }
}

func main() {
   if len(os.Args) == 1 {
      fmt.Println("*** Using default JSON record.")
   } else {
      JsonRecord = os.Args[1]
   }

   jsonMap := make(map[string]interface{})
   err := json.Unmarshal([]byte(JsonRecord), &jsonMap)
   if err != nil {
      fmt.Println(err)
      return
   }

   exploreMap(jsonMap)
   typeSwitch(jsonMap)
}
