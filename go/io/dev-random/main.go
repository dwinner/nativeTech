package main

import (
   "encoding/binary"
   "fmt"
   "os"
)

func main() {
   file, err := os.Open("/dev/random")
   defer func(file *os.File) {
      err := file.Close()
      if err != nil {
         os.Exit(-1)
      }
   }(file)
   if err != nil {
      fmt.Println(err)
      return
   }

   var seed int64
   _ = binary.Read(file, binary.LittleEndian, &seed)
   fmt.Println("Seed:", seed)
}
