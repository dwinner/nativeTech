package main

import (
   "bufio"
   "fmt"
   "io"
   "os"
)

func charByChar(aFile string) error {
   openedFd, err := os.Open(aFile)
   if err != nil {
      return err
   }

   defer func(file *os.File) {
      err := file.Close()
      if err != nil {
         os.Exit(-1)
      }
   }(openedFd)

   reader := bufio.NewReader(openedFd)
   for {
      readRune, _, err := reader.ReadRune()
      if err == io.EOF {
         fmt.Print(string(readRune))
         break
      } else if err != nil {
         fmt.Printf("Error reading rune %s", err)
         return err
      }

      fmt.Print(string(readRune))
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
