package main

import (
   "fmt"
   "log"
   "net/http"
   "os"
   "time"

   "github.com/gorilla/websocket"
)

var PORT = ":1234"

var upgrader = websocket.Upgrader{
   ReadBufferSize:  1024,
   WriteBufferSize: 1024,
   CheckOrigin: func(r *http.Request) bool {
      return true
   },
}

func rootHandler(aWriter http.ResponseWriter, _ *http.Request) {
   _, _ = fmt.Fprintf(aWriter, "Welcome!\n")
   _, _ = fmt.Fprintf(aWriter, "Please use /ws for WebSocket!")
}

func wsHandler(aWriter http.ResponseWriter, aRequest *http.Request) {
   log.Println("Connection from:", aRequest.Host)
   conn, err := upgrader.Upgrade(aWriter, aRequest, nil)
   if err != nil {
      log.Println("upgrader.Upgrade:", err)
      return
   }

   defer func(conn *websocket.Conn) {
      err := conn.Close()
      if err != nil {
         panic(err)
      }
   }(conn)

   for {
      messageType, message, err := conn.ReadMessage()
      if err != nil {
         log.Println("From", aRequest.Host, "read", err)
         break
      }

      log.Print("Received: ", string(message))
      err = conn.WriteMessage(messageType, message)
      if err != nil {
         log.Println("WriteMessage:", err)
         break
      }
   }
}

func main() {
   arguments := os.Args
   if len(arguments) != 1 {
      PORT = ":" + arguments[1]
   }

   mux := http.NewServeMux()
   server := &http.Server{
      Addr:         PORT,
      Handler:      mux,
      IdleTimeout:  10 * time.Second,
      ReadTimeout:  time.Second,
      WriteTimeout: time.Second,
   }

   mux.Handle("/", http.HandlerFunc(rootHandler))
   mux.Handle("/ws", http.HandlerFunc(wsHandler))

   log.Println("Listening to TCP Port", PORT)
   err := server.ListenAndServe()
   if err != nil {
      log.Println(err)
      return
   }
}
