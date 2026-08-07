package main

type AllInts interface {
   ~int
}

// Node is linked node struct
type Node[T AllInts] struct {
   value      T
   prev, next *Node[T]
}

// DoublyLinkedList is struct for doubly linked list
type DoublyLinkedList[T AllInts] struct {
   head, tail *Node[T]
}

// NewDoublyLinkedList creates new doubly linked list
func NewDoublyLinkedList[T AllInts]() *DoublyLinkedList[T] {
   return &DoublyLinkedList[T]{}
}

// O(1) time | O(1) space
func (aLinkedList *DoublyLinkedList[T]) setHead(node *Node[T]) {
   if aLinkedList.head == nil {
      aLinkedList.head = node
      aLinkedList.tail = node
      return
   }

   aLinkedList.insertBefore(aLinkedList.head, node)
}

// O(1) time | O(1) space
func (aLinkedList *DoublyLinkedList[T]) setTail(node *Node[T]) {
   if aLinkedList.tail == nil {
      aLinkedList.setHead(node)
      return
   }

   aLinkedList.insertAfter(aLinkedList.tail, node)
}

// O(1) time | O(1) space
func (aLinkedList *DoublyLinkedList[T]) insertBefore(node, nodeToInsert *Node[T]) {
   if nodeToInsert == aLinkedList.head && nodeToInsert == aLinkedList.tail {
      return
   }

   aLinkedList.remove(nodeToInsert)
   nodeToInsert.prev = node.prev
   nodeToInsert.next = node
   if node.prev == nil {
      aLinkedList.head = nodeToInsert
   } else {
      node.prev.next = nodeToInsert
   }

   node.prev = nodeToInsert
}

// O(1) time | O(1) space
func (aLinkedList *DoublyLinkedList[T]) insertAfter(node, nodeToInsert *Node[T]) {
   if nodeToInsert == aLinkedList.head && nodeToInsert == aLinkedList.tail {
      return
   }

   aLinkedList.remove(nodeToInsert)
   nodeToInsert.prev = node
   nodeToInsert.next = node.next
   if node.next == nil {
      aLinkedList.tail = nodeToInsert
   } else {
      node.next.prev = nodeToInsert
   }

   node.next = nodeToInsert
}

// O(p) time | O(1) space
func (aLinkedList *DoublyLinkedList[T]) insertAtPosition(position int, nodeToInsert *Node[T]) {
   if position == 1 {
      aLinkedList.setHead(nodeToInsert)
      return
   }

   node := aLinkedList.head
   currentPosition := 1
   for node != nil && currentPosition != position {
      node = node.next
      currentPosition++
   }

   if node != nil {
      aLinkedList.insertBefore(node, nodeToInsert)
   } else {
      aLinkedList.setTail(nodeToInsert)
   }
}

// O(n) time | O(1) space
func (aLinkedList *DoublyLinkedList[T]) removeNodesWithValue(value T) {
   node := aLinkedList.head
   for node != nil {
      nodeToRemove := node
      node = node.next
      if nodeToRemove.value == value {
         aLinkedList.remove(nodeToRemove)
      }
   }
}

// O(1) time | O(1) space
func (aLinkedList *DoublyLinkedList[T]) remove(node *Node[T]) {
   if node == aLinkedList.head {
      aLinkedList.head = aLinkedList.head.next
   }

   if node == aLinkedList.tail {
      aLinkedList.tail = aLinkedList.tail.prev
   }

   aLinkedList.removeNodeBindings(node)
}

// O(n) time | O(1) space
func (aLinkedList *DoublyLinkedList[T]) containsNodeWithValue(value T) bool {
   node := aLinkedList.head
   for node != nil && node.value != value {
      node = node.next
   }

   return node != nil
}

func (aLinkedList *DoublyLinkedList[T]) removeNodeBindings(node *Node[T]) {
   if node.prev != nil {
      node.prev.next = node.next
   }

   if node.next != nil {
      node.next.prev = node.prev
   }

   node.prev = nil
   node.next = nil
}
