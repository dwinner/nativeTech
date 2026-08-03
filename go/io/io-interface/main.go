package main

import (
   "bufio"
   "fmt"
   "io"
)

type S1 struct {
   F1 int
   F2 string
}

type S2 struct {
   F1   S1
   text []byte
}

// Using pointer to S1 for changes to be persistent when the method exits
func (aStruct *S1) Read(bytes []byte) (bytesLen int, err error) {
   fmt.Print("Give me your name: ")
   bytesLen, err = fmt.Scanln(&bytes)
   if err != nil {
      return bytesLen, err
   }

   aStruct.F2 = string(bytes)
   return len(bytes), nil
}

func (aStruct *S1) Write(bytes []byte) (bytesLen int, err error) {
   if aStruct.F1 < 0 {
      return -1, nil
   }

   for i := 0; i < aStruct.F1; i++ {
      fmt.Printf("%s ", bytes)
   }

   fmt.Println()
   return aStruct.F1, nil
}

func (aStruct S2) eof() bool {
   return len(aStruct.text) == 0
}

func (aStruct *S2) readByte() byte {
   // this function assumes that eof() check was done before
   temp := aStruct.text[0]
   aStruct.text = aStruct.text[1:]
   return temp
}

func (aStruct *S2) Read(bytes []byte) (aLen int, err error) {
   if aStruct.eof() {
      err = io.EOF
      return 0, err
   }

   bytesLen := len(bytes)
   if bytesLen > 0 {
      for aLen < bytesLen {
         bytes[aLen] = aStruct.readByte()
         aLen++
         if aStruct.eof() {
            aStruct.text = aStruct.text[0:0]
            break
         }
      }
   }

   return aLen, nil
}

func main() {
   s1var := S1{4, "Hello"}
   fmt.Println(s1var)
   buf := make([]byte, 2)
   _, err := s1var.Read(buf)
   if err != nil {
      fmt.Println(err)
      return
   }

   fmt.Println("Read:", s1var.F2)
   _, _ = s1var.Write([]byte("Hello There!"))
   s2var := S2{F1: s1var, text: []byte("Hello world!!")}

   // Read s2var.text
   reader := bufio.NewReader(&s2var)
   for {
      readLen, err := reader.Read(buf)
      if err == io.EOF {
         break
      } else if err != nil {
         fmt.Println("*", err)
         break
      }

      fmt.Println("**", readLen, string(buf[:readLen]))
   }
}
