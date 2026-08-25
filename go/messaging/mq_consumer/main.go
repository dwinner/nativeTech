package main

import (
   "fmt"

   amqp "github.com/rabbitmq/amqp091-go"
)

func main() {
   fmt.Println("RabbitMQ consumer")
   mqConnection, err := amqp.Dial("amqp://guest:guest@localhost:5672/")
   if err != nil {
      fmt.Println("Failed Initializing Broker Connection")
      panic(err)
   }

   defer func(aConnection *amqp.Connection) {
      err := aConnection.Close()
      if err != nil {
         panic(err)
      }
   }(mqConnection)

   mqChannel, err := mqConnection.Channel()
   if err != nil {
      fmt.Println(err)
   }

   defer func(aChannel *amqp.Channel) {
      err := aChannel.Close()
      if err != nil {
         panic(err)
      }
   }(mqChannel)

   messages, err := mqChannel.Consume("Go", "", true, false, false, false, nil)
   if err != nil {
      fmt.Println(err)
   }

   forever := make(chan bool)
   go func() {
      for message := range messages {
         fmt.Printf("Received: %s\n", message.Body)
      }
   }()

   fmt.Println("Connected to the RabbitMQ server!")
   <-forever
}
