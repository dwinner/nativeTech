package main

import (
   "fmt"
   "math/rand"
   "net"
   "os"
   "strconv"
   "strings"
   "time"
)

func random(min, max int) int {
   return rand.Intn(max-min) + min
}

func main() {
   arguments := os.Args
   if len(arguments) == 1 {
      fmt.Println("Please provide a port number!")
      return
   }

   PORT := ":" + arguments[1]
   udpAddr, err := net.ResolveUDPAddr("udp4", PORT)
   if err != nil {
      fmt.Println(err)
      return
   }

   udpConn, err := net.ListenUDP("udp4", udpAddr)
   if err != nil {
      fmt.Println(err)
      return
   }

   defer func(aUdpConn *net.UDPConn) {
      _ = aUdpConn.Close()
   }(udpConn)

   buffer := make([]byte, 1024)
   rand.Seed(time.Now().Unix())

   for {
      readLen, addr, err := udpConn.ReadFromUDP(buffer)
      fmt.Print("-> ", string(buffer[0:readLen-1]))
      if strings.TrimSpace(string(buffer[0:readLen])) == "STOP" {
         fmt.Println("Exiting UDP server!")
         return
      }

      data := []byte(strconv.Itoa(random(1, 1001)))
      fmt.Printf("data: %s\n", string(data))
      _, err = udpConn.WriteToUDP(data, addr)
      if err != nil {
         fmt.Println(err)
         return
      }
   }
}
