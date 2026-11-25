package main

import (
   "strings"

   "github.com/spf13/pflag"
   "github.com/spf13/viper"
)

const linesKey = "lines"
const bytesKey = "bytes"
const charsKey = "chars"
const longestStrKey = "longest"
const words = "words"
const files = "files"

func configureArgs() {
   pflag.BoolP(linesKey, "l", false, "The count of lines")
   pflag.BoolP(bytesKey, "c", false, "The count of bytes")
   pflag.BoolP(charsKey, "m", false, "The count of symbols")
   pflag.BoolP(longestStrKey, "L", false, "The longest string")
   pflag.BoolP(words, "w", false, "The count of words")
   pflag.StringP(files, "f", "", "Files to process")
   pflag.Parse()
   _ = viper.BindPFlags(pflag.CommandLine)
}

func parseFilesArgument(files string) []string {
   fileCollection := strings.Split(files, ",")
   for idx := range fileCollection {
      fileCollection[idx] = strings.Trim(fileCollection[idx], `"`)
   }

   return fileCollection
}
