package main

import (
   "fmt"
)

type node[T any] struct {
   Data T
   next *node[T]
}

type list[T any] struct {
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

func main() {
   var myList list[int]
   fmt.Println(myList)
   myList.add(12)
   myList.add(9)
   myList.add(3)
   myList.add(9)

   // Print all elements
   current := myList.start
   for {
      fmt.Println("*", current)
      if current == nil {
         break
      }

      current = current.next
   }
}
