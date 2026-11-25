/**
 * wc_buf_channel <filename-1> <filename-2> ... <filename-n>:
 */

package main

import (
   "fmt"
   "os"
   "sync"
   "wcutils"
)

func main() {
   args := os.Args
   argsLen := len(args)
   if argsLen == 1 {
      fmt.Printf("Usage: wc-simple <filename-1> <filename-2> ... <filename-n>\n")
      os.Exit(0)
   }

   fileChannel := make(chan string, argsLen-1)
   for _, file := range args[1:] {
      select {
      case fileChannel <- file:
         fmt.Printf("About to process '%s'\n", file)
      default:
         fmt.Println("No more files to process")
      }
   }

   var waitGrp sync.WaitGroup
   for {
      select {
      case file := <-fileChannel:
         fmt.Printf("Received '%s'\n", file)
         waitGrp.Go(func() {
            wcRun(file)
         })
      default:
         waitGrp.Wait()
         close(fileChannel)
         fmt.Println("No more read items")
         return
      }
   }
}

func wcRun(aFile string) {
   // Count chars
   charCount, err := wcutils.CountByChar(aFile)
   if err != nil {
      fmt.Println(err)
   } else {
      fmt.Printf("Count of chars in file '%s': '%d'\n", aFile, charCount)
   }

   // Count lines
   lineCount, err := wcutils.CountByLine(aFile)
   if err != nil {
      fmt.Println(err)
   } else {
      fmt.Printf("Count of lines in file '%s': '%d'\n", aFile, lineCount)
   }

   // Count words
   wordCount, err := wcutils.CountByWord(aFile)
   if err != nil {
      fmt.Println(err)
   } else {
      fmt.Printf("Count of words in file '%s': '%d'\n", aFile, wordCount)
   }
}
