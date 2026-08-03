package main

import (
   "bufio"
   "fmt"
   "io"
   "os"
   "regexp"
)

func wordByWord(aFile string) error {
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
   nonWsRegExpr := regexp.MustCompile("\\S+")
   for {
      line, err := reader.ReadString('\n')
      if err == io.EOF {
         if len(line) != 0 {
            words := nonWsRegExpr.FindAllString(line, -1)
            for i := range words {
               fmt.Println(words[i])
            }
         }

         break
      } else if err != nil {
         fmt.Printf("Error reading file %s", err)
         return err
      }

      words := nonWsRegExpr.FindAllString(line, -1)
      for i := range words {
         fmt.Println(words[i])
      }
   }

   return nil
}

func main() {
   args := os.Args
   if len(args) == 1 {
      fmt.Printf("usage: byWord <file1> [<file2> ...]\n")
      return
   }

   for _, file := range args[1:] {
      err := wordByWord(file)
      if err != nil {
         fmt.Println(err)
      }
   }
}
