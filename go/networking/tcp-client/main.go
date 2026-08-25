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
      fmt.Println("Please provide host:port.")
      return
   }

   connect := arguments[1]
   conn, err := net.Dial("tcp", connect)
   if err != nil {
      fmt.Println(err)
      os.Exit(5)
   }

   reader := bufio.NewReader(os.Stdin)
   for {
      fmt.Print(">> ")
      text, _ := reader.ReadString('\n')
      _, _ = fmt.Fprintf(conn, "%s\n", text)
      message, _ := bufio.NewReader(conn).ReadString('\n')
      fmt.Print("->: " + message)
      if strings.TrimSpace(text) == "STOP" {
         fmt.Println("TCP client exiting...")
         return
      }
   }
}
