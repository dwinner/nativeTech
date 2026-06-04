package main

import "fmt"

type Entry struct {
   Name    string
   Surname string
   Year    int
}

// Initialized by Go
func zeroEntry() Entry {
   return Entry{}
}

// Initialized by the user
func initEntry(name, sirName string, year int) Entry {
   if year < 2000 {
      return Entry{
         Name:    name,
         Surname: sirName,
         Year:    2000,
      }
   }

   return Entry{
      Name:    name,
      Surname: sirName,
      Year:    year,
   }
}

// Initialized by Go - returns pointer
func zeroEntryPtr() *Entry {
   pEntry := &Entry{}
   return pEntry
}

// Initialized by the user - returns pointer
func initEntryPtr(aName, aSirName string, aYear int) *Entry {
   if len(aSirName) == 0 {
      return &Entry{
         Name:    aName,
         Surname: "Unknown",
         Year:    aYear,
      }
   }

   return &Entry{
      Name:    aName,
      Surname: aSirName,
      Year:    aYear,
   }
}

func main() {
   s1 := zeroEntry()
   p1 := zeroEntryPtr()
   fmt.Println("s1:", s1, "p1:", *p1)

   s2 := initEntry("Mihalis", "Tsoukalos", 2024)
   p2 := initEntryPtr("Mihalis", "Tsoukalos", 2024)
   fmt.Println("s2:", s2, "p2:", *p2)
   fmt.Println("Year:", s1.Year, s2.Year, p1.Year, p2.Year)
   pS := new(Entry)
   fmt.Println("pS:", pS)
}
