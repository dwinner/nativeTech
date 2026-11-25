package main

import (
   "fmt"
   "sort"
   "time"
)

type Person struct {
   Id          int
   Name        string
   SirName     string
   DateOfBirth time.Time
}

type PersonCollection []Person

func (aPersonColl PersonCollection) Len() int {
   return len(aPersonColl)
}

func (aPersonColl PersonCollection) Less(i, j int) bool {
   cmpRes := aPersonColl[i].DateOfBirth.Compare(aPersonColl[j].DateOfBirth)
   if cmpRes == -1 || cmpRes == 0 {
      return true
   }

   return false
}

func (aPersonColl PersonCollection) Swap(i, j int) {
   aPersonColl[i], aPersonColl[j] = aPersonColl[j], aPersonColl[i]
}

func main() {
   personColl := []Person{
      {1, "Den", "Winner", time.Date(1985, 3, 21, 0, 0, 0, 0, time.Local)},
      {1, "Vladimir", "Lankin", time.Date(1975, 7, 23, 0, 0, 0, 0, time.Local)},
      {1, "Constantin", "Radugin", time.Date(1995, 8, 2, 0, 0, 0, 0, time.Local)},
   }

   fmt.Println("Before: ", personColl)
   sort.Sort(PersonCollection(personColl))
   fmt.Println("After: ", personColl)
}
