/**
 * wc_semaphore <filename-1> <filename-2> ... <filename-n>:
 */

package main

import (
   "context"
   "fmt"
   "io"
   "os"
   "wcutils"

   "golang.org/x/sync/semaphore"
)

const WorkerCount int64 = 1

func main() {
   args := os.Args
   argsLen := len(args)
   if argsLen == 1 {
      fmt.Printf("Usage: wc-simple <filename-1> <filename-2> ... <filename-n>\n")
      os.Exit(0)
   }

   results := make([]WordCountItem, argsLen-1)
   numberedGate := semaphore.NewWeighted(WorkerCount)
   ctx := context.TODO()
   for idx, file := range args[1:] {
      err := numberedGate.Acquire(ctx, 1)
      if err != nil {
         fmt.Println("Cannot acquire semaphore:", err)
         break
      }

      go func(fileToProc string) {
         defer numberedGate.Release(1)
         item := wcRun(fileToProc)
         results[idx] = item
         fmt.Printf("Done with '%s'\n", fileToProc)
      }(file)
   }

   // Now, block until all workers have finished
   err := numberedGate.Acquire(ctx, WorkerCount)
   if err != nil {
      fmt.Println(err)
   }

   // Save results to file
   {
      outFile, err := os.Create("output.log")
      if err != nil {
         fmt.Println("Cannot create file", err)
         return
      }

      defer func(outFile *os.File) {
         err := outFile.Close()
         if err != nil {
            os.Exit(-1)
         }
      }(outFile)

      for _, result := range results {
         charCountStr := fmt.Sprintf("Count of chars in file '%s': '%d'\n", result.Filename, result.CharCount)
         lineCountStr := fmt.Sprintf("Count of lines in file '%s': '%d'\n", result.Filename, result.LineCount)
         wordCountStr := fmt.Sprintf("Count of words in file '%s': '%d'\n", result.Filename, result.WordCount)
         _, _ = io.WriteString(outFile, charCountStr)
         _, _ = io.WriteString(outFile, lineCountStr)
         _, _ = io.WriteString(outFile, wordCountStr)
      }
   }
}

func wcRun(aFile string) (wcItem WordCountItem) {
   // Count chars
   charCount, err := wcutils.CountByChar(aFile)
   if err != nil {
      fmt.Println(err)
      charCount = 0
   }

   // Count lines
   lineCount, err := wcutils.CountByLine(aFile)
   if err != nil {
      fmt.Println(err)
      lineCount = 0
   }

   // Count words
   wordCount, err := wcutils.CountByWord(aFile)
   if err != nil {
      fmt.Println(err)
      wordCount = 0
   }

   return WordCountItem{
      CharCount: charCount,
      WordCount: wordCount,
      LineCount: lineCount,
      Filename:  aFile,
   }
}
