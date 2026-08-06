package main

import (
   "bufio"
   "fmt"
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

   scanner := bufio.NewScanner(fileD)
   for scanner.Scan() {
      text := scanner.Text()
      fmt.Println(text)
   }

   if err := scanner.Err(); err != nil {
      _, err = fmt.Fprintln(os.Stderr, "reading standard input:", err)
      return err
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
