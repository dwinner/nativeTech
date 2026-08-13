package wcutils

import (
   "bufio"
   "fmt"
   "io"
   "os"
)

func CountByChar(aFile string) (charCount int64, err error) {
   file, err := os.Open(aFile)
   if err != nil {
      return 0, err
   }

   defer func(fd *os.File) {
      err := fd.Close()
      if err != nil {
         fmt.Println(err.Error())
         os.Exit(-1)
      }
   }(file)

   reader := bufio.NewReader(file)
   charCount = 0
   for {
      line, err := reader.ReadString('\n')
      if err == io.EOF {
         if len(line) != 0 {
            for range line {
               charCount++
            }
         }

         break
      } else if err != nil {
         fmt.Printf("Error reading file %s", err)
         return 0, err
      }

      for range line {
         charCount++
      }
   }

   return charCount, nil
}
