package main

import (
   "fmt"
   "net"
   "os"
   "strings"
)

func main() {
   arguments := os.Args
   if len(arguments) == 1 {
      fmt.Println("Please provide a port number!")
      return
   }

   SERVER := "localhost" + ":" + arguments[1]
   tcpAddr, err := net.ResolveTCPAddr("tcp", SERVER)
   if err != nil {
      fmt.Println(err)
      return
   }

   listener, err := net.ListenTCP("tcp", tcpAddr)
   if err != nil {
      fmt.Println(err)
      return
   }

   buffer := make([]byte, 1024)
   conn, err := listener.Accept()
   if err != nil {
      fmt.Println(err)
      return
   }

   for {
      readLen, err := conn.Read(buffer)
      if err != nil {
         fmt.Println(err)
         return
      }

      if strings.TrimSpace(string(buffer[0:readLen])) == "STOP" {
         fmt.Println("Exiting TCP server!")
         _ = conn.Close()
         return
      }

      fmt.Print("> ", string(buffer[0:readLen-1]), "\n")
      _, err = conn.Write(buffer)
      if err != nil {
         fmt.Println(err)
         return
      }
   }
}
