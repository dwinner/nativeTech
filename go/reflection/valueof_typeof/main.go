package main

import (
   "fmt"
   "reflect"
)

type Secret struct {
   Username string
   Password string
}

type Record struct {
   Field1 string
   Field2 float64
   Field3 Secret
}

func main() {
   rec := Record{"String value", -12.123, Secret{"Mihalis", "Tsoukalos"}}
   valueOfRec := reflect.ValueOf(rec)
   fmt.Println("String value:", valueOfRec.String())
   typeOfRec := valueOfRec.Type()
   fmt.Printf("i Type: %s\n", typeOfRec)
   fmt.Printf("The %d fields of %s are\n", valueOfRec.NumField(), typeOfRec)

   for i := 0; i < valueOfRec.NumField(); i++ {
      fmt.Printf("\t%s ", typeOfRec.Field(i).Name)
      fmt.Printf("\twith type: %s ", valueOfRec.Field(i).Type())
      fmt.Printf("\tand value _%v_\n", valueOfRec.Field(i).Interface())

      // Check whether there are other structures embedded in Record
      kind := reflect.TypeOf(valueOfRec.Field(i).Interface()).Kind()

      // Need to convert it to string in order to compare it
      if kind.String() == "struct" {
         fmt.Println(valueOfRec.Field(i).Type())
      }

      // Same as before but using the internal value
      if kind == reflect.Struct {
         fmt.Println(valueOfRec.Field(i).Type())
      }
   }
}
