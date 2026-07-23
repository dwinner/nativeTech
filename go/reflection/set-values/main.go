package main

import (
   "fmt"
   "reflect"
)

type T struct {
   F1 int
   F2 string
   F3 float64
}

func main() {
   rec := T{1, "F2", 3.0}
   fmt.Println("A:", rec)

   valueOfRec := reflect.ValueOf(&rec).Elem()
   fmt.Println("String value:", valueOfRec.String())
   typeOfRec := valueOfRec.Type()
   for i := 0; i < valueOfRec.NumField(); i++ {
      field := valueOfRec.Field(i)
      fieldName := typeOfRec.Field(i).Name
      fmt.Printf("%d: %s %s = %v\n", i, fieldName, field.Type(), field.Interface())

      kind := reflect.TypeOf(valueOfRec.Field(i).Interface()).Kind()
      if kind == reflect.Int {
         valueOfRec.Field(i).SetInt(-100)
      } else if kind == reflect.String {
         valueOfRec.Field(i).SetString("Changed!")
      }
   }

   fmt.Println("A:", rec)
}
