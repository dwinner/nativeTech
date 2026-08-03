package main

import (
   "bufio"
   "fmt"
   "io"
   "os"
)

func lineByLine(aFile string) error {
   fileD, err := os.Open(aFile)
   if err != nil {
      return err
   }

   defer func(fileD *os.File) {
      err := fileD.Close()
      if err != nil {
         os.Exit(-1)
      }
   }(fileD)

   reader := bufio.NewReader(fileD)
   for {
      line, err := reader.ReadString('\n')
      if err == io.EOF {
         if len(line) != 0 {
            fmt.Println(line)
         }

         break
      }

      if err != nil {
         fmt.Printf("error reading file %s", err)
         return err
      }

      fmt.Print(line)
   }

   return nil
}

func main() {
   args := os.Args
   if len(args) == 1 {
      fmt.Printf("usage: byLine <file1> [<file2> ...]\n")
      return
   }

   for _, file := range args[1:] {
      err := lineByLine(file)
      if err != nil {
         fmt.Println(err)
      }
   }
}
