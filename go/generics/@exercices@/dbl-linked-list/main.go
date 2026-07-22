package main

// Node is linked node struct
type Node struct {
   value      int
   prev, next *Node
}

// DoublyLinkedList is struct for doubly linked list
type DoublyLinkedList struct {
   head, tail *Node
}

// NewDoublyLinkedList creates new doubly linked list
func NewDoublyLinkedList() *DoublyLinkedList {
   return &DoublyLinkedList{}
}

// O(1) time | O(1) space
func (dll *DoublyLinkedList) setHead(node *Node) {
   if dll.head == nil {
      dll.head = node
      dll.tail = node
      return
   }
   dll.insertBefore(dll.head, node)
}

// O(1) time | O(1) space
func (dll *DoublyLinkedList) setTail(node *Node) {
   if dll.tail == nil {
      dll.setHead(node)
      return
   }
   dll.insertAfter(dll.tail, node)
}

// O(1) time | O(1) space
func (dll *DoublyLinkedList) insertBefore(node, nodeToInsert *Node) {
   if nodeToInsert == dll.head && nodeToInsert == dll.tail {
      return
   }
   dll.remove(nodeToInsert)
   nodeToInsert.prev = node.prev
   nodeToInsert.next = node
   if node.prev == nil {
      dll.head = nodeToInsert
   } else {
      node.prev.next = nodeToInsert
   }
   node.prev = nodeToInsert
}

// O(1) time | O(1) space
func (dll *DoublyLinkedList) insertAfter(node, nodeToInsert *Node) {
   if nodeToInsert == dll.head && nodeToInsert == dll.tail {
      return
   }
   dll.remove(nodeToInsert)
   nodeToInsert.prev = node
   nodeToInsert.next = node.next
   if node.next == nil {
      dll.tail = nodeToInsert
   } else {
      node.next.prev = nodeToInsert
   }
   node.next = nodeToInsert
}

// O(p) time | O(1) space
func (dll *DoublyLinkedList) insertAtPosition(position int, nodeToInsert *Node) {
   if position == 1 {
      dll.setHead(nodeToInsert)
      return
   }
   node := dll.head
   currnetPosition := 1
   for node != nil && currnetPosition != position {
      node = node.next
      currnetPosition++
   }
   if node != nil {
      dll.insertBefore(node, nodeToInsert)
   } else {
      dll.setTail(nodeToInsert)
   }
}

// O(n) time | O(1) space
func (dll *DoublyLinkedList) removeNodesWithValue(value int) {
   node := dll.head
   for node != nil {
      nodeToRemove := node
      node = node.next
      if nodeToRemove.value == value {
         dll.remove(nodeToRemove)
      }
   }
}

// O(1) time | O(1) space
func (dll *DoublyLinkedList) remove(node *Node) {
   if node == dll.head {
      dll.head = dll.head.next
   }
   if node == dll.tail {
      dll.tail = dll.tail.prev
   }
   dll.removeNodeBindings(node)
}

// O(n) time | O(1) space
func (dll *DoublyLinkedList) containsNodeWithValue(value int) bool {
   node := dll.head
   for node != nil && node.value != value {
      node = node.next
   }
   return node != nil
}

func (dll *DoublyLinkedList) removeNodeBindings(node *Node) {
   if node.prev != nil {
      node.prev.next = node.next
   }
   if node.next != nil {
      node.next.prev = node.prev
   }
   node.prev = nil
   node.next = nil
}
