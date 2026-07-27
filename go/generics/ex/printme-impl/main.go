package main

import (
   "cmp"
   "fmt"
)

type AllInts interface {
   ~int
}

type node[T AllInts] struct {
   Data T
   next *node[T]
}

type list[T AllInts] struct {
   start *node[T]
}

func (aLinkedList *list[T]) add(data T) {
   newNode := node[T]{
      Data: data,
      next: nil,
   }

   if aLinkedList.start == nil {
      aLinkedList.start = &newNode
      return
   }

   if aLinkedList.start.next == nil {
      aLinkedList.start.next = &newNode
      return
   }

   temp := aLinkedList.start
   aLinkedList.start = aLinkedList.start.next
   aLinkedList.add(data)
   aLinkedList.start = temp
}

// PrintMe Print the list
func (aLinkedList *list[T]) PrintMe() {
   current := aLinkedList.start
   for {
      fmt.Println("*", current)
      if current == nil {
         break
      }

      current = current.next
   }
}

func (aLinkedList *list[T]) delete(anElement T) bool {
   current := aLinkedList.start
   for {
      if current == nil {
         return false
      }

      data := current.Data
      cmpRes := cmp.Compare(data, anElement)
      if cmpRes == 0 {
         for current != nil && current.next != nil {
            current.Data = current.next.Data
            current.next = current.next.next
            current = current.next
         }

         return true
      }

      current = current.next
   }
}

func (aLinkedList *list[T]) search(anElement T) *T {
   current := aLinkedList.start
   for {
      if current == nil {
         return nil
      }

      data := current.Data
      cmpRes := cmp.Compare(data, anElement)
      if cmpRes == 0 {
         return &data
      }

      current = current.next
   }
}

func main() {
   var myList list[int]
   fmt.Println(myList)

   // Add some data
   myList.add(12)
   myList.add(9)
   myList.add(3)
   myList.add(9)

   // Print all elements
   myList.PrintMe()

   // Search element '3'
   element := myList.search(3)
   if element != nil {
      fmt.Println("Found: ", *element)
   } else {
      fmt.Println("Not found")
   }

   // Search element '2'
   element = myList.search(2)
   if element != nil {
      fmt.Println("Found: ", *element)
   } else {
      fmt.Println("Not found")
   }

   // Delete element '3'
   deleted := myList.delete(3)
   fmt.Println("Deleted: ", deleted)
   myList.PrintMe()
}
