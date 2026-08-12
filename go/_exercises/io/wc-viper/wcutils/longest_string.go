package wcutils

import (
   "bufio"
   "fmt"
   "io"
   "os"
   "regexp"
)

func ComputeLongestStringLen(files ...string) (length int) {
   maxLen := 0
   for _, file := range files {
      wordLen, err := computeLongestWord(file)
      if err != nil {
         continue
      } else if wordLen > maxLen {

         maxLen = wordLen

      }
   }

   return maxLen
}

func computeLongestWord(aFile string) (length int, err error) {
   fileD, err := os.Open(aFile)
   if err != nil {
      return 0, err
   }

   defer func(fileD *os.File) {
      err := fileD.Close()
      if err != nil {
         fmt.Println(err.Error())
         os.Exit(-1)
      }
   }(fileD)

   bufReader := bufio.NewReader(fileD)
   wordRegExpr := regexp.MustCompile("\\S+")
   var maxLen = 0
   for {
      line, err := bufReader.ReadString('\n')
      if err == io.EOF {
         words := wordRegExpr.FindAllString(line, -1)
         currentMaxLen := computeMaxWordLen(words)
         if currentMaxLen > maxLen {
            maxLen = currentMaxLen
         }

         break
      } else if err != nil {
         fmt.Printf("Error reading file '%s'\n", err)
         return 0, err
      }

      words := wordRegExpr.FindAllString(line, -1)
      currentMaxLen := computeMaxWordLen(words)
      if currentMaxLen > maxLen {
         maxLen = currentMaxLen
      }
   }

   return maxLen, nil
}

func computeMaxWordLen(words []string) int {
   maxLen := 0
   for _, word := range words {
      length := len(word)
      if length > maxLen {
         maxLen = length
      }
   }

   return maxLen
}
