package wcutils

import (
   "bufio"
   "fmt"
   "io"
   "os"
)

func CountByBytes(aFile string) (byteCount int64, err error) {
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

   bufReader := bufio.NewReader(fileD)
   byteCount = 0
   for {
      bytes, err := bufReader.ReadBytes('\n')
      if err == io.EOF {
         byteCount += int64(len(bytes))
         break
      } else if err != nil {
         fmt.Println(err.Error())
         break
      } else {
         byteCount += int64(len(bytes))
      }
   }

   return byteCount, nil
}
