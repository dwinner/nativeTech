package main

import (
   "fmt"
   "strconv"
)

type record struct {
   Field1 int
   Field2 string
}

func main() {
   var arrOfRec []record
   for i := 0; i < 10; i++ {
      text := "text" + strconv.Itoa(i)
      temp := record{Field1: i, Field2: text}
      arrOfRec = append(arrOfRec, temp)
   }

   // Accessing the fields of the first element
   fmt.Println("Index 0:", arrOfRec[0].Field1, arrOfRec[0].Field2)
   fmt.Println("Number of structures:", len(arrOfRec))
   sum := 0
   for _, val := range arrOfRec {
      sum += val.Field1
   }

   fmt.Println("Sum:", sum)
}
