package main

import (
   "bufio"
   "fmt"
   "net"
   "os"
   "strconv"
   "strings"
)

var count = 0

func handleConnection(aConnection net.Conn, aCount int) {
   fmt.Print(".")
   netData, err := bufio.NewReader(aConnection).ReadString('\n')
   if err != nil {
      fmt.Println(err)
      return
   }

   for {
      temp := strings.TrimSpace(netData)
      if temp == "STOP" {
         break
      }

      fmt.Println(temp)
      counter := "Client number: " + strconv.Itoa(aCount) + "\n"
      _, err := aConnection.Write([]byte(counter))
      if err != nil {
         fmt.Println(err)
         return
      }
   }

   defer func(aConnection net.Conn) {
      err := aConnection.Close()
      if err != nil {
         fmt.Println(err)
         return
      }
   }(aConnection)
}

func main() {
   arguments := os.Args
   if len(arguments) == 1 {
      fmt.Println("Please provide a port number!")
      os.Exit(5)
   }

   PORT := ":" + arguments[1]
   listener, err := net.Listen("tcp4", PORT)
   if err != nil {
      fmt.Println(err)
      return
   }

   defer func(listener net.Listener) {
      err := listener.Close()
      if err != nil {
         fmt.Println(err)
         return
      }
   }(listener)

   for {
      newConnection, err := listener.Accept()
      if err != nil {
         fmt.Println(err)
         return
      }

      go handleConnection(newConnection, count)
      count++
   }
}
