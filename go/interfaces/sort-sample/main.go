package main

import (
   "fmt"
   "sort"
)

type Size struct {
   F1 int
   F2 string
   F3 int
}

// Person We want to sort Person records based on the value of Size.
// F1 Which is Size.
// F1 as F3 is a Size structure
type Person struct {
   F1 int
   F2 string
   F3 Size
}

type PersonSlice []Person

// Len Implementing sort.Interface for PersonSlice
func (aPersonSlice PersonSlice) Len() int {
   return len(aPersonSlice)
}

// Less What field to use for comparing
func (aPersonSlice PersonSlice) Less(i, j int) bool {
   return aPersonSlice[i].F3.F1 < aPersonSlice[j].F3.F1
}

func (aPersonSlice PersonSlice) Swap(i, j int) {
   aPersonSlice[i], aPersonSlice[j] = aPersonSlice[j], aPersonSlice[i]
}

func main() {
   data := []Person{
      {1, "One", Size{1, "Person_1", 10}},
      {2, "Two", Size{2, "Person_2", 20}},
      {-1, "Two", Size{-1, "Person_3", -20}},
   }
   fmt.Println("Before:", data)
   sort.Sort(PersonSlice(data))
   fmt.Println("After:", data)

   // Reverse sorting works automatically
   sort.Sort(sort.Reverse(PersonSlice(data)))
   fmt.Println("Reverse:", data)
}
