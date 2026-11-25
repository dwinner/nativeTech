/**
 * wc_shared_mem <filename-1> <filename-2> ... <filename-n>:
 */

package main

import (
   "fmt"
   "os"
   "sync"
   "wcutils"
)

var totalWc = NewTotalWc(0, 0, 0, &sync.Mutex{})

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
         totalWc.Print()
         return
      }
   }
}

func wcRun(aFile string) {
   charCount := countChars(aFile)
   lineCount := countLines(aFile)
   wordCount := countWords(aFile)
   totalWc.UpdateCounters(charCount, lineCount, wordCount)
}

func countWords(aFile string) (wordCount int64) {
   wordCount, err := wcutils.CountByWord(aFile)
   if err != nil {
      fmt.Println(err)
      wordCount = 0
   }

   return wordCount
}

func countLines(aFile string) (lineCount int64) {
   lineCount, err := wcutils.CountByLine(aFile)
   if err != nil {
      fmt.Println(err)
      lineCount = 0
   }

   return lineCount
}

func countChars(aFile string) (charCount int64) {
   charCount, err := wcutils.CountByChar(aFile)
   if err != nil {
      fmt.Println(err)
      charCount = 0
   }

   return charCount
}
