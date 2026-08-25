package main

import (
   "fmt"

   amqp "github.com/rabbitmq/amqp091-go"
)

func main() {
   fmt.Println("RabbitMQ producer")
   qConn, err := amqp.Dial("amqp://guest:guest@localhost:5672/")
   if err != nil {
      fmt.Println("amqp.Dial():", err)
      return
   }

   defer func(aConnection *amqp.Connection) {
      err := aConnection.Close()
      if err != nil {
         fmt.Println("Error while closing connection", err)
         return
      }
   }(qConn)

   qChannel, err := qConn.Channel()
   if err != nil {
      fmt.Println(err)
      return
   }

   defer func(aChannel *amqp.Channel) {
      err := aChannel.Close()
      if err != nil {
         fmt.Println("Error while closing channel", err)
         return
      }
   }(qChannel)

   declaredQueue, err := qChannel.QueueDeclare("Go", false, false, false, false, nil)
   if err != nil {
      fmt.Println(err)
      return
   }

   fmt.Println("Queue:", declaredQueue)
   message := "Writing to RabbitMQ!"
   err = qChannel.PublishWithContext(nil,
      "",
      "Go",
      false,
      false,
      amqp.Publishing{ContentType: "text/plain", Body: []byte(message)},
   )

   if err != nil {
      fmt.Println(err)
      return
   }

   fmt.Println("Message published to Queue!")
}
