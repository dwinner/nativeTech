package main

import (
   "bufio"
   "fmt"
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

   nonWsRegExpr := regexp.MustCompile("\\S+")
   scanner := bufio.NewScanner(fileD)
   for scanner.Scan() {
      line := scanner.Text()
      words := nonWsRegExpr.FindAllString(line, -1)
      for i := range words {
         fmt.Print(words[i], " ")
      }

      fmt.Println()
   }

   if err = scanner.Err(); err != nil {
      _, _ = fmt.Fprintln(os.Stderr, "Reading failed", err)
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
