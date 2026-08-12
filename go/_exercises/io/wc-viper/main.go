/**
 * wc-viper - UNIX-like word count cli-tool
 * wc-viper -l -f "filename1","filename2",... Print the line count
 * wc-viper -c -f "filename1","filename2",... Print the byte count вывести количество байт
 * wc-viper -m -f "filename1","filename2",... Print the char count
 * wc-viper -L -f "filename1","filename2",... Print the length of the longest string
 * wc-viper -w -f "filename1","filename2",... Print the word count
 */

package main

import (
   "fmt"
   "os"
   "wcutils"

   "github.com/spf13/viper"
)

func main() {
   // Configure cl-arguments
   configureArgs()

   // Get all set flags
   boLineCountSet := viper.IsSet(linesKey)
   boByteCountSet := viper.IsSet(bytesKey)
   boCharCountSet := viper.IsSet(charsKey)
   boWordCountSet := viper.IsSet(words)
   boLongestStrSet := viper.IsSet(longestStrKey)
   filesArg := viper.GetString(files)
   fileColl := parseFilesArgument(filesArg)
   if fileColl == nil || len(fileColl) == 0 {
      fmt.Println("No files to process")
      os.Exit(0)
   }

   if boLineCountSet {
      for _, file := range fileColl {
         lineCount, err := wcutils.CountByLine(file)
         if err == nil {
            fmt.Printf("The count of lines in file '%s': '%d'\n", file, lineCount)
         }
      }

      fmt.Println()
   }

   if boByteCountSet {
      for _, file := range fileColl {
         byteCount, err := wcutils.CountByBytes(file)
         if err == nil {
            fmt.Printf("The count of bytes in file '%s': '%d'\n", file, byteCount)
         }
      }

      fmt.Println()
   }

   if boCharCountSet {
      for _, file := range fileColl {
         charCount, err := wcutils.CountByChar(file)
         if err == nil {
            fmt.Printf("The count of chars in file '%s': '%d'\n", file, charCount)
         }
      }

      fmt.Println()
   }

   if boWordCountSet {
      for _, file := range fileColl {
         wordCount, err := wcutils.CountByWord(file)
         if err == nil {
            fmt.Printf("The count of words in file '%s': '%d'\n", file, wordCount)
         }
      }

      fmt.Println()
   }

   if boLongestStrSet {
      maxLen := wcutils.ComputeLongestStringLen(fileColl...)
      fmt.Printf("The longest string length is '%d'\n", maxLen)
      fmt.Println()
   }
}
