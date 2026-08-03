package main

import (
   "encoding/json"
   "fmt"
   "os"

   "github.com/spf13/viper"
)

type ConfigStructure struct {
   MacPass     string `mapstructure:"macos"`
   LinuxPass   string `mapstructure:"linux"`
   WindowsPass string `mapstructure:"windows"`
   PostHost    string `mapstructure:"postgres"`
   MySQLHost   string `mapstructure:"mysql"`
   MongoHost   string `mapstructure:"mongodb"`
}

func PrettyPrint(anAnyType any) (err error) {
   bytesWritten, err := json.MarshalIndent(anAnyType, "", "  ")
   if err == nil {
      fmt.Println(string(bytesWritten))
   }

   return
}

var CONFIG = "myConfig.json"

func main() {
   if len(os.Args) == 1 {
      fmt.Println("Using default file", CONFIG)
   } else {
      CONFIG = os.Args[1]
   }

   viper.SetConfigType("json")
   viper.SetConfigFile(CONFIG)
   fmt.Printf("Using config: %s\n", viper.ConfigFileUsed())
   _ = viper.ReadInConfig()

   if viper.IsSet("macos") {
      fmt.Println("macos:", viper.Get("macos"))
   } else {
      fmt.Println("macos not set!")
   }

   if viper.IsSet("active") {
      value := viper.GetBool("active")
      if value {
         postgres := viper.Get("postgres")
         mysql := viper.Get("mysql")
         mongo := viper.Get("mongodb")
         fmt.Println("P:", postgres, "My:", mysql, "Mo:", mongo)
      }
   } else {
      fmt.Println("active is not set!")
   }

   if !viper.IsSet("DoesNotExist") {
      fmt.Println("DoesNotExist is not set!")
   }

   var cfgStruct ConfigStructure
   err := viper.Unmarshal(&cfgStruct)
   if err != nil {
      fmt.Println(err)
      return
   }

   _ = PrettyPrint(cfgStruct)
}
