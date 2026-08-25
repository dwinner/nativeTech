package main

import (
   "bufio"
   "fmt"
   "net"
   "os"
   "strings"
)

func main() {
   arguments := os.Args
   if len(arguments) == 1 {
      fmt.Println("Please provide a host:port string")
      return
   }

   CONNECT := arguments[1]
   udpAddr, err := net.ResolveUDPAddr("udp4", CONNECT)
   udpConn, err := net.DialUDP("udp4", nil, udpAddr)
   if err != nil {
      fmt.Println(err)
      return
   }

   fmt.Printf("The UDP server is %s\n", udpConn.RemoteAddr().String())
   defer func(aUdpConn *net.UDPConn) {
      _ = aUdpConn.Close()
   }(udpConn)

   reader := bufio.NewReader(os.Stdin)
   for {
      fmt.Print(">> ")
      text, _ := reader.ReadString('\n')
      data := []byte(text + "\n")
      _, err = udpConn.Write(data)
      if strings.TrimSpace(string(data)) == "STOP" {
         fmt.Println("Exiting UDP client!")
         return
      }

      if err != nil {
         fmt.Println(err)
         return
      }

      buffer := make([]byte, 1024)
      readLen, _, err := udpConn.ReadFromUDP(buffer)
      if err != nil {
         fmt.Println(err)
         return
      }

      fmt.Printf("Reply: %s\n", string(buffer[0:readLen]))
   }
}
