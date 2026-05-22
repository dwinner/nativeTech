package main

import (
   "fmt"
   "os"
   "strconv"
)

func main() {
   if len(os.Args) == 1 {
      fmt.Println("Print provide an integer.")
      return
   }

   intVal, err := strconv.Atoi(os.Args[1])
   if err != nil {
      fmt.Println(err)
      return
   }

   // Using strconv.Itoa()
   strInput := strconv.Itoa(intVal)
   fmt.Printf("strconv.Itoa() %s of type %T\n", strInput, strInput)

   // Using strconv.FormatInt
   strInput = strconv.FormatInt(int64(intVal), 10)
   fmt.Printf("strconv.FormatInt() %s of type %T\n", strInput, strInput)

   // Using string()
   strInput = string(rune(intVal))
   fmt.Printf("string() %s of type %T\n", strInput, strInput)
}
