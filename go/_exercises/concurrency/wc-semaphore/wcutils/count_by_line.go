package wcutils

import (
   "bufio"
   "fmt"
   "io"
   "os"
)

func CountByLine(aFile string) (lineCount int64, err error) {
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
   lineCount = 0
   for {
      line, err := reader.ReadString('\n')
      if err == io.EOF {
         if len(line) != 0 {
            lineCount++
         }

         break
      }

      if err != nil {
         fmt.Printf("error reading file %s", err)
         return 0, err
      }

      lineCount++
   }

   return lineCount, nil
}
