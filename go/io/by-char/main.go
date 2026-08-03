package main

import (
   "bufio"
   "fmt"
   "io"
   "os"
)

func charByChar(aFile string) error {
   file, err := os.Open(aFile)
   if err != nil {
      return err
   }

   defer func(file *os.File) {
      err := file.Close()
      if err != nil {
         os.Exit(-1)
      }
   }(file)

   reader := bufio.NewReader(file)
   for {
      line, err := reader.ReadString('\n')
      if err == io.EOF {
         if len(line) != 0 {
            for _, char := range line {
               fmt.Println(string(char))
            }
         }
         break
      } else if err != nil {
         fmt.Printf("Error reading file %s", err)
         return err
      }

      for _, x := range line {
         fmt.Println(string(x))
      }
   }

   return nil
}

func main() {
   args := os.Args
   if len(args) == 1 {
      fmt.Printf("usage: byCharacter <file1> [<file2> ...]\n")
      return
   }

   for _, file := range args[1:] {
      err := charByChar(file)
      if err != nil {
         fmt.Println(err)
      }
   }
}
