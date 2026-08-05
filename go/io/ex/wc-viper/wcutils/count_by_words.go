package wcutils

import (
   "bufio"
   "fmt"
   "io"
   "os"
   "regexp"
)

func CountByWord(aFile string) (wordCount int64, err error) {
   fileD, err := os.Open(aFile)
   if err != nil {
      return 0, err
   }

   defer func(fd *os.File) {
      err := fd.Close()
      if err != nil {
         fmt.Println(err.Error())
         os.Exit(-1)
      }
   }(fileD)

   reader := bufio.NewReader(fileD)
   nonWsRegExpr := regexp.MustCompile("\\S+")
   wordCount = 0
   for {
      line, err := reader.ReadString('\n')
      if err == io.EOF {
         if len(line) != 0 {
            words := nonWsRegExpr.FindAllString(line, -1)
            for range words {
               wordCount++
            }
         }

         break
      } else if err != nil {
         fmt.Printf("Error reading file %s", err)
         return 0, err
      }

      words := nonWsRegExpr.FindAllString(line, -1)
      for range words {
         wordCount++
      }
   }

   return wordCount, nil
}
