package main

import (
   "bufio"
   "fmt"
   "net"
   "os"
   "strings"
)

func main() {
   const HostAndPort = "localhost:1234"
   tcpAddr, err := net.ResolveTCPAddr("tcp4", HostAndPort)
   if err != nil {
      fmt.Println("Error resolving ", err)
      return
   }

   tcpConn, err := net.DialTCP("tcp4", nil, tcpAddr)
   if err != nil {
      fmt.Println("Dial error ", err)
      return
   }

   reader := bufio.NewReader(os.Stdin)
   for {
      fmt.Println("Enter the range in format min-max >> ")
      input, err := reader.ReadString('\n')
      _, err = fmt.Fprintf(tcpConn, input+"\n")
      if err != nil {
         fmt.Println(err)
         return
      }

      message, _ := bufio.NewReader(tcpConn).ReadString('\n')
      fmt.Print("->: ", message)
      if strings.TrimSpace(input) == "STOP" {
         fmt.Println("TCP client exiting...")
         err := tcpConn.Close()
         if err != nil {
            fmt.Println(err)
            return
         }

         return
      }
   }
}
