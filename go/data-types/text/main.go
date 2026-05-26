package main

import "fmt"

func main() {
   aString := "Hello World! €"
   fmt.Println("First byte", string(aString[0]))

   // Runes
   // A myRune
   myRune := '€'
   fmt.Println("As an int32 value:", myRune)

   // Convert Runes to text
   fmt.Printf("As a string: %s and as a character: %c\n", myRune, myRune)

   // Print an existing string as runes
   for _, val := range aString {
      fmt.Printf("%x ", val)
   }

   fmt.Println()

   // String to myRune Array
   // myRune := []myRune(aString)
   // fmt.Printf("myRune %U\n", myRune)

   // Rune array to string
   // runeArray := []myRune{'1', '2', '3'}
   // s := string(runeArray)
   // fmt.Println(s)

   // Print an existing string as characters
   for _, val := range aString {
      fmt.Printf("%c", val)
   }

   fmt.Println()
}
