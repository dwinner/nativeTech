/**
 * UNSUPPORTED:
 * wc-simple - UNIX-like word count cli-tool
 * wc-simple -l <filename> вывести количество строк
 * wc-simple -c <filename> вывести количество байт
 * wc-simple -m <filename> вывести количество символов
 * wc-simple -L <filename> вывести длину самой длинной строки
 * wc-simple -w <filename> вывести количество слов
 * SUPPORTED:
 * wc-simple <filename-1> <filename-2> ... <filename-n>:
				line-count     word-count     symbol-count filename-1
   			etc.
*/

package main

import (
   "fmt"
   "os"
   "wcutils"
)

func main() {
   args := os.Args
   if len(args) == 1 {
      fmt.Printf("Usage: wc-simple <filename-1> <filename-2> ... <filename-n>\n")
      os.Exit(0)
   }

   for _, file := range args[1:] {
      // Count chars
      charCount, err := wcutils.CountByChar(file)
      if err != nil {
         fmt.Println(err)
      } else {
         fmt.Printf("Count of chars in file '%s': '%d'\n", file, charCount)
      }

      // Count lines
      lineCount, err := wcutils.CountByLine(file)
      if err != nil {
         fmt.Println(err)
      } else {
         fmt.Printf("Count of lines in file '%s': '%d'\n", file, lineCount)
      }

      // Count words
      wordCount, err := wcutils.CountByWord(file)
      if err != nil {
         fmt.Println(err)
      } else {
         fmt.Printf("Count of words in file '%s': '%d'\n", file, wordCount)
      }
   }
}
