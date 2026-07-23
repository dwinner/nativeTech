package main

import "fmt"

type Secret struct {
   SecretValue string
}

type Entry struct {
   F1 int
   F2 string
   F3 Secret
}

func TestStruct(iface interface{}) {
   // type switch
   switch typeSwitch := iface.(type) {
   case Secret:
      fmt.Println("Secret type")
   case Entry:
      fmt.Println("Entry type")
   default:
      fmt.Printf("Not supported type: %T\n", typeSwitch)
   }
}

func Learn(iface interface{}) {
   switch typeSwitch := iface.(type) {
   default:
      fmt.Printf("Data type: %T\n", typeSwitch)
   }
}

func main() {
   entry := Entry{100, "F2", Secret{"myPassword"}}
   TestStruct(entry)
   TestStruct(entry.F3)
   TestStruct("A string")

   Learn(12.23)
   Learn('€')
}
