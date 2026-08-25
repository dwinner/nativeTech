package main

import (
   "bufio"
   "fmt"
   "net"
   "os"
   "strings"
   "time"
)

func main() {
   arguments := os.Args
   if len(arguments) == 1 {
      fmt.Println("Please provide port number")
      return
   }

   PORT := ":" + arguments[1]
   listener, err := net.Listen("tcp", PORT)
   if err != nil {
      fmt.Println(err)
      return
   }

   defer func(aListener net.Listener) {
      _ = aListener.Close()
   }(listener)

   conn, err := listener.Accept()
   if err != nil {
      fmt.Println(err)
      return
   }

   for {
      netData, err := bufio.NewReader(conn).ReadString('\n')
      if err != nil {
         fmt.Println(err)
         return
      }

      if strings.TrimSpace(netData) == "STOP" {
         fmt.Println("Exiting TCP server!")
         return
      }

      fmt.Print("-> ", netData)
      nowTime := time.Now()
      myTime := nowTime.Format(time.RFC3339) + "\n"
      _, _ = conn.Write([]byte(myTime))
   }
}
