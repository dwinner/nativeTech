package main

import (
   "fmt"
   "os"
   "strconv"
)

func main() {
   arguments := os.Args
   if len(arguments) == 1 {
      fmt.Println("Need an integer value.")
      return
   }

   index := arguments[1]
   intVal, err := strconv.Atoi(index)
   if err != nil {
      fmt.Println(err)
      return
   }

   fmt.Println("Using index", intVal)

   aSlice := []int{0, 1, 2, 3, 4, 5, 6, 7, 8}
   fmt.Println("Original slice:", aSlice)

   // Delete element at index intVal
   cutIndex := len(aSlice) - 1
   if intVal > cutIndex {
      fmt.Println("Cannot delete element", intVal)
      return
   }

   // The ... operator auto expands aSlice[intVal+1:] so that
   // its elements can be appended to aSlice[:intVal] one by one
   aSlice = append(aSlice[:intVal], aSlice[intVal+1:]...)
   fmt.Println("After 1st deletion:", aSlice)

   // Delete element at index intVal
   if intVal > cutIndex {
      fmt.Println("Cannot delete element", intVal)
      return
   }

   // Replace element at index intVal with last element
   aSlice[intVal] = aSlice[cutIndex]

   // Remove last element
   aSlice = aSlice[:cutIndex]
   fmt.Println("After 2nd deletion:", aSlice)
}
