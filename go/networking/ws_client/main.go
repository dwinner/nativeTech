package main

import (
   "bufio"
   "fmt"
   "log"
   "net/url"
   "os"
   "os/signal"
   "syscall"
   "time"

   "github.com/gorilla/websocket"
)

var (
   SERVER       = ""
   PATH         = ""
   TimesWait    = 0
   TimesWaitMax = 5
   in           = bufio.NewReader(os.Stdin)
)

func getInput(anInput chan<- string) {
   result, err := in.ReadString('\n')
   if err != nil {
      log.Println(err)
      return
   }

   anInput <- result
}

func main() {
   arguments := os.Args
   if len(arguments) != 3 {
      fmt.Println("Need SERVER + PATH!")
      return
   }

   SERVER = arguments[1]
   PATH = arguments[2]
   fmt.Println("Connecting to:", SERVER, "at", PATH)

   interrupt := make(chan os.Signal, 1)
   signal.Notify(interrupt, os.Interrupt)

   input := make(chan string, 1)
   go getInput(input)

   connectUrl := url.URL{Scheme: "ws", Host: SERVER, Path: PATH}
   conn, _, err := websocket.DefaultDialer.Dial(connectUrl.String(), nil)
   if err != nil {
      log.Println("Error:", err)
      return
   }

   defer func(conn *websocket.Conn) {
      err := conn.Close()
      if err != nil {
         panic(err)
      }
   }(conn)

   done := make(chan struct{})
   go func() {
      defer close(done)
      for {
         _, message, err := conn.ReadMessage()
         if err != nil {
            log.Println("ReadMessage() error:", err)
            return
         }

         log.Printf("Received: %s", message)
      }
   }()

   for {
      select {
      case <-time.After(4 * time.Second):
         log.Println("Please give me input!", TimesWait)
         TimesWait++
         if TimesWait > TimesWaitMax {
            syscall.Exit(int(syscall.SIGINT))
            //syscall.Kill(syscall.Getpid(), syscall.SIGINT)
         }
      case <-done:
         return
      case message := <-input:
         err := conn.WriteMessage(websocket.TextMessage, []byte(message))
         if err != nil {
            log.Println("Write error:", err)
            return
         }

         TimesWait = 0
         go getInput(input)
      case <-interrupt:
         log.Println("Caught interrupt signal - quitting!")
         err := conn.WriteMessage(websocket.CloseMessage, websocket.FormatCloseMessage(websocket.CloseNormalClosure, ""))
         if err != nil {
            log.Println("Write close error:", err)
            return
         }

         select {
         case <-done:
         case <-time.After(2 * time.Second):
         }

         return
      }
   }
}
