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
   // Read socket path
   if len(os.Args) == 1 {
      fmt.Println("Need socket path")
      return
   }

   socketPath := os.Args[1]
   conn, err := net.Dial("unix", socketPath)
   if err != nil {
      fmt.Println(err)
      return
   }

   defer func(aConnection net.Conn) {
      _ = aConnection.Close()
   }(conn)

   for {
      reader := bufio.NewReader(os.Stdin)
      fmt.Print(">> ")
      text, _ := reader.ReadString('\n')
      _, err = conn.Write([]byte(text))
      if err != nil {
         fmt.Println("Write:", err)
         break
      }

      buf := make([]byte, 256)
      readLen, err := conn.Read(buf[:])
      if err != nil {
         fmt.Println(err, readLen)
         return
      }

      fmt.Print("Read: ", string(buf[0:readLen]))
      if strings.TrimSpace(text) == "STOP" {
         fmt.Println("Exiting UNIX domain socket client!")
         return
      }

      time.Sleep(5 * time.Second)
   }
}
