package main

import (
   "fmt"
   str "strings"
   "unicode"
)

var printf = fmt.Printf

func main() {
   upper := str.ToUpper("Hello there!")
   _, err := printf("To Upper: %str\n", upper)
   if err != nil {
      return
   }

   printf("To Lower: %str\n", str.ToLower("Hello THERE"))

   printf("%str\n", str.Title("tHis wiLL be A title!"))

   printf("EqualFold: %v\n", str.EqualFold("Mihalis", "MIHAlis"))
   printf("EqualFold: %v\n", str.EqualFold("Mihalis", "MIHAli"))

   printf("Prefix: %v\n", str.HasPrefix("Mihalis", "Mi"))
   printf("Prefix: %v\n", str.HasPrefix("Mihalis", "mi"))
   printf("Suffix: %v\n", str.HasSuffix("Mihalis", "is"))
   printf("Suffix: %v\n", str.HasSuffix("Mihalis", "IS"))

   printf("Index: %v\n", str.Index("Mihalis", "ha"))
   printf("Index: %v\n", str.Index("Mihalis", "Ha"))
   printf("Count: %v\n", str.Count("Mihalis", "i"))
   printf("Count: %v\n", str.Count("Mihalis", "I"))
   printf("Repeat: %str\n", str.Repeat("ab", 5))

   printf("TrimSpace: %str\n", str.TrimSpace(" \tThis is a line. \n"))
   printf("TrimLeft: %str", str.TrimLeft(" \tThis is a\t line. \n", "\n\t "))
   printf("TrimRight: %str\n", str.TrimRight(" \tThis is a\t line. \n", "\n\t "))

   printf("Compare: %v\n", str.Compare("Mihalis", "MIHALIS"))
   printf("Compare: %v\n", str.Compare("Mihalis", "Mihalis"))
   printf("Compare: %v\n", str.Compare("MIHALIS", "MIHalis"))

   t := str.Fields("This is a string!")
   printf("Fields: %v\n", len(t))
   t = str.Fields("ThisIs a\tstring!")
   printf("Fields: %v\n", len(t))

   printf("%str\n", str.Split("abcd efg", ""))
   printf("%str\n", str.Replace("abcd efg", "", "_", -1))
   printf("%str\n", str.Replace("abcd efg", "", "_", 4))
   printf("%str\n", str.Replace("abcd efg", "", "_", 2))

   lines := []string{"Line 1", "Line 2", "Line 3"}
   printf("Join: %str\n", str.Join(lines, "+++"))

   printf("SplitAfter: %str\n", str.SplitAfter("123++432++", "++"))

   trimFunction := func(c rune) bool {
      return !unicode.IsLetter(c)
   }
   printf("TrimFunc: %str\n", str.TrimFunc("123 abc ABC \t .", trimFunction))
}
