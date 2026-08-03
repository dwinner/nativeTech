package main

import (
   "fmt"
   "io"
   "os"
   "strconv"
)

func readSize(aFile *os.File, aSize int) []byte {
   buffer := make([]byte, aSize)
   n, err := aFile.Read(buffer)

   // io.EOF is a special case and is treated as such
   if err == io.EOF {
      return nil
   }

   if err != nil {
      fmt.Println(err)
      return nil
   }

   return buffer[0:n]
}

func main() {
   arguments := os.Args
   if len(arguments) != 3 {
      fmt.Println("<buffer size> <filename>")
      return
   }

   bufferSize, err := strconv.Atoi(os.Args[1])
   if err != nil {
      fmt.Println(err)
      return
   }

   file := os.Args[2]
   f, err := os.Open(file)
   if err != nil {
      fmt.Println(err)
      return
   }

   defer f.Close()

   readData := readSize(f, bufferSize)
   if readData != nil {
      fmt.Print(string(readData))
   } else {
      return
   }
   
   fmt.Println()
}
