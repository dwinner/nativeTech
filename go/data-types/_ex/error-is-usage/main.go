package main

import (
   "errors"
   "fmt"
   "os"
   "strconv"
)

type NullOperandError struct {
   err string
}

func (err NullOperandError) Error() string {
   return err.err
}

// Custom error message with errors.New()
func check(a, b int) error {
   if a == 0 && b == 0 {
      return NullOperandError{err: fmt.Sprintf("%d and %d are zero", a, b)}
   }

   return nil
}

// Custom error message with fmt.Errorf()
func formattedError(a, b int) error {
   if a == 0 && b == 0 {
      return fmt.Errorf("a %d and b %d. UserID: %d", a, b, os.Getuid())
   }

   return nil
}

func main() {
   err := check(0, 10)
   if errors.Is(err, NullOperandError{}) {
      fmt.Println("Null operand error occurred")
   } else {
      fmt.Println(err)
   }

   err = check(0, 0)
   if errors.Is(err, NullOperandError{}) {
      fmt.Println("Custom error detected!")
   } else {
      fmt.Println("Ok")
   }

   err = formattedError(0, 0)
   if err != nil {
      fmt.Println(err)
   }

   intVal, err := strconv.Atoi("-123")
   if err == nil {
      fmt.Println("Int value is", intVal)
   }

   intVal, err = strconv.Atoi("Y123")
   if err != nil {
      fmt.Println(err)
   }
}
