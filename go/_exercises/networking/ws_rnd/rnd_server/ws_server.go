package main

import (
   "fmt"
   "log"
   "math/rand"
   "net/http"
   "strconv"
   "strings"
   "time"

   "github.com/gorilla/websocket"
)

const Port = ":1234"

var upgrader = websocket.Upgrader{
   ReadBufferSize:  1024,
   WriteBufferSize: 1024,
   CheckOrigin: func(request *http.Request) bool {
      return true
   },
}

func main() {
   mux := http.NewServeMux()
   server := &http.Server{
      Addr:         Port,
      Handler:      mux,
      IdleTimeout:  10 * time.Second,
      ReadTimeout:  time.Second,
      WriteTimeout: time.Second,
   }

   mux.Handle("/", http.HandlerFunc(rootHandler))
   mux.Handle("/ws", http.HandlerFunc(wsHandler))

   log.Println("Listening to TCP Port", Port)
   err := server.ListenAndServe()
   if err != nil {
      log.Println(err)
      return
   }
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

   defer func(aConnection *websocket.Conn) {
      err := aConnection.Close()
      if err != nil {
         panic(err)
      }
   }(conn)

   interop(conn, aRequest)
}

func interop(aWsConn *websocket.Conn, aRequest *http.Request) {
   for {
      messageType, message, err := aWsConn.ReadMessage()
      if err != nil {
         log.Println("From", aRequest.Host, "read", err)
         break
      }

      rawInput := strings.TrimSpace(string(message))
      intValue, err := strconv.Atoi(rawInput)
      if err != nil {
         log.Println("Malformed input")
         continue
      }

      // Generate random numbers
      rndValues := make([]string, intValue)
      for i := range intValue {
         randomValue := genRnd(0, 100)
         rndValues[i] = strconv.Itoa(randomValue)
      }

      // Send random numbers as string
      response := strings.Join(rndValues, ",")

      log.Print("Received: ", response)
      err = aWsConn.WriteMessage(messageType, []byte(response))
      if err != nil {
         log.Println("WriteMessage:", err)
         break
      }
   }
}

func genRnd(aMin, aMax int) int {
   rdnGen := rand.New(rand.NewSource(time.Now().UnixNano()))
   rndNum := rdnGen.Intn(aMax-aMin+1) + aMin

   return rndNum
}
