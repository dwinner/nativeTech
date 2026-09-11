package table

import (
   "bufio"
   "fmt"
   "math/rand"
   "os"
)

var BufferSize int
var Filesize int

func random(min, max int) int {
   return rand.Intn(max-min) + min
}

func CreateBuffer(aByteBuf *[]byte, count int) {
   *aByteBuf = make([]byte, count)
   if count == 0 {
      return
   }

   for range count {
      intByte := byte(random(50, 100))
      if len(*aByteBuf) > count {
         return
      }

      *aByteBuf = append(*aByteBuf, intByte)
   }
}

func Create(dstFile string, aLen, filesize int) error {
   _, err := os.Stat(dstFile)
   if err == nil {
      return fmt.Errorf("file %s already exists", dstFile)
   }

   file, err := os.Create(dstFile)
   if err != nil {
      fmt.Printf("error opening file %s", err)
      return err
   }

   _ = file.Close()

   file, err = os.OpenFile(dstFile, os.O_WRONLY, 0655)
   if err != nil {
      return err
   }

   defer func(file *os.File) {
      _ = file.Close()
   }(file)

   writer := bufio.NewWriterSize(file, aLen)
   buf := make([]byte, 0)
   CreateBuffer(&buf, aLen)
   buf = buf[:aLen]
   for {
      _, err := writer.Write(buf)
      if err != nil {
         return err
      }

      if filesize < 0 {
         break
      }

      filesize = filesize - len(buf)
   }

   return err
}

func CountChars(filename string, count int) int {
   buf := make([]byte, 0)
   CreateBuffer(&buf, count)
   file, err := os.Open(filename)
   if err != nil {
      fmt.Printf("error opening file %s", err)
      return -1
   }

   defer func(file *os.File) {
      _ = file.Close()
   }(file)

   size := 0
   for {
      n, err := file.Read(buf)
      size = size + n
      if err != nil {
         break
      }
   }

   return size
}
