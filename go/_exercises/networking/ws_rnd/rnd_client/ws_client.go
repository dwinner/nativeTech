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
   Server       = "localhost:1234"
   Path         = "/ws"
   TimesWait    = 0
   TimesWaitMax = 5
   in           = bufio.NewReader(os.Stdin)
)

func main() {
   fmt.Println("Connecting to:", Server, "at", Path)

   interrupt := make(chan os.Signal, 1)
   signal.Notify(interrupt, os.Interrupt)

   input := make(chan string, 1)
   go getInput(input)

   connectUrl := url.URL{Scheme: "ws", Host: Server, Path: Path}
   conn, _, err := websocket.DefaultDialer.Dial(connectUrl.String(), nil)
   if err != nil {
      log.Println("Error:", err)
      return
   }

   defer func(aWsConn *websocket.Conn) {
      err := aWsConn.Close()
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

   interop(conn, done, input, interrupt)
}

func interop(
    conn *websocket.Conn,
    done chan struct{},
    input chan string,
    interrupt chan os.Signal) {
   for {
      select {
      case <-time.After(4 * time.Second):
         log.Println("Please give me input!", TimesWait)
         TimesWait++
         if TimesWait > TimesWaitMax {
            syscall.Exit(int(syscall.SIGINT))
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

func getInput(anInput chan<- string) {
   result, err := in.ReadString('\n')
   if err != nil {
      log.Println(err)
      return
   }

   anInput <- result
}
