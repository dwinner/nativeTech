package main

import (
   "bufio"
   "errors"
   "fmt"
   "log"
   "math/rand"
   "net"
   "strconv"
   "strings"
   "sync"
   "time"
)

type RndServer struct {
   listener  net.Listener
   quit      chan struct{}
   waitGroup *sync.WaitGroup
}

func (aRndServer *RndServer) serve() {
   for {
      conn, err := aRndServer.listener.Accept()
      if err != nil {
         log.Printf("Accept error: %v", err)
         continue
      }

      aRndServer.waitGroup.Add(1)
      go func(aConnection net.Conn) {
         defer aRndServer.waitGroup.Done()

         // Handle a new incoming connection
         handleConnection(aConnection)
      }(conn)

      select {
      // Check quit channel
      case <-aRndServer.quit:
         log.Printf("Connection %v closed by shutdown", conn.RemoteAddr())
         aRndServer.waitGroup.Done()
      default:
         // Connection is still active, continue handling
      }
   }
}

func NewServer(anAddress string) *RndServer {
   listener, err := net.Listen("tcp4", anAddress)
   if err != nil {
      log.Fatalf("Failed to listen: %v", err)
   }

   server := &RndServer{
      listener:  listener,
      quit:      make(chan struct{}),
      waitGroup: &sync.WaitGroup{},
   }

   go server.serve()
   return server
}

func (aRndServer *RndServer) Stop() {
   log.Println("Received shutdown signal, stopping server...")

   // Close the channel, it's the signal that all goroutines should be stopped
   close(aRndServer.quit)

   // Close the listener
   err := aRndServer.listener.Close()
   if err != nil {
      log.Fatal("Error while closing the listener")
      return
   }

   // Waiting all handlers are done
   aRndServer.waitGroup.Wait()

   log.Println("Server stopped gracefully")
}

// Handle new incoming connection
func handleConnection(aConnection net.Conn) {
   fmt.Print(".")
   clientInput, err := bufio.NewReader(aConnection).ReadString('\n')
   if err != nil {
      fmt.Println(err)
      return
   }

   defer func(anInputConn net.Conn) {
      err := anInputConn.Close()
      if err != nil {
         fmt.Println(err)
         return
      }
   }(aConnection)

   for {
      input := strings.TrimSpace(clientInput)
      if input == "STOP" {
         break
      }

      // Handle client input
      minVal, maxVal, err := handleInput(input)
      if err != nil {
         fmt.Println(err)
         return
      }

      // Response to the connected client
      rndNum := genRnd(minVal, maxVal)
      response := []byte(strconv.Itoa(rndNum) + "\n")
      _, err = aConnection.Write(response)
      if err != nil {
         fmt.Println(err)
         return
      }
   }
}

func handleInput(anInput string) (int, int, error) {
   rawRndInput := strings.Split(anInput, "-")
   if len(rawRndInput) == 2 {
      aMinValue, err := strconv.Atoi(rawRndInput[0])
      if err != nil {
         return 0, 0, err
      }

      aMaxValue, err := strconv.Atoi(rawRndInput[1])
      if err != nil {
         return 0, 0, err
      }

      return aMinValue, aMaxValue, nil
   }

   return 0, 0, errors.New("wrong input")
}

func genRnd(aMin, aMax int) int {
   rdnGen := rand.New(rand.NewSource(time.Now().UnixNano()))
   rndNum := rdnGen.Intn(aMax-aMin+1) + aMin

   return rndNum
}
