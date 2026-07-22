package main

import "testing"

func TestSingleNodeHead(t *testing.T) {
   dll := NewDoublyLinkedList[int]()
   node := newNode /*[int]*/ (1)

   dll.setHead(node)
   expectingSingleNode(t, dll, node)
}

func TestSingleNodeTail(t *testing.T) {
   dll := NewDoublyLinkedList[int]()
   node := newNode(1)

   dll.setTail(node)
   expectingSingleNode(t, dll, node)
}

func TestRemoveHead(t *testing.T) {
   dll := NewDoublyLinkedList[int]()
   node := newNode(1)

   dll.setHead(node)
   dll.remove(node)
   expectingEmpty(t, dll)
}

func TestRemoveTail(t *testing.T) {
   dll := NewDoublyLinkedList[int]()
   node := newNode(1)

   dll.setTail(node)
   dll.remove(node)
   expectingEmpty(t, dll)
}

func TestRemoveNodesWithValueHead(t *testing.T) {
   dll := NewDoublyLinkedList[int]()
   node := newNode(1)

   dll.setHead(node)
   dll.removeNodesWithValue(1)
   expectingEmpty(t, dll)
}

func TestRemoveNodesWithValueTail(t *testing.T) {
   dll := NewDoublyLinkedList[int]()
   node := newNode(1)

   dll.setTail(node)
   dll.removeNodesWithValue(1)
   expectingEmpty(t, dll)
}

func TestInsertAtPosition(t *testing.T) {
   dll := NewDoublyLinkedList[int]()
   node := newNode(1)

   dll.insertAtPosition(1, node)
   expectingSingleNode(t, dll, node)
}

func TestSetHeadAndTail(t *testing.T) {
   dll := NewDoublyLinkedList[int]()
   first := newNode(1)
   second := newNode(2)

   dll.setHead(first)
   dll.setTail(second)
   expectingHeadTail(t, dll, first, second)
}

func TestInsertAfterHead(t *testing.T) {
   dll := NewDoublyLinkedList[int]()
   first := newNode(1)
   second := newNode(2)

   dll.setHead(first)
   dll.insertAfter(first, second)
   expectingHeadTail(t, dll, first, second)
}

func TestInsertBeforeHead(t *testing.T) {
   dll := NewDoublyLinkedList[int]()
   first := newNode(1)
   second := newNode(2)

   dll.setHead(first)
   dll.insertBefore(first, second)
   expectingHeadTail(t, dll, second, first)
}

func TestInsertAfterTail(t *testing.T) {
   dll := NewDoublyLinkedList[int]()
   first := newNode(1)
   second := newNode(2)

   dll.setTail(first)
   dll.insertAfter(first, second)
   expectingHeadTail(t, dll, first, second)
}

func TestInsertBeforeTail(t *testing.T) {
   dll := NewDoublyLinkedList[int]()
   first := newNode(1)
   second := newNode(2)

   dll.setHead(first)
   dll.insertBefore(first, second)
   expectingHeadTail(t, dll, second, first)
}

func TestInsertAtPositionHeadTail(t *testing.T) {
   dll := NewDoublyLinkedList[int]()
   first := newNode(1)
   second := newNode(2)

   dll.insertAtPosition(1, first)
   dll.insertAtPosition(2, second)
   expectingHeadTail(t, dll, first, second)
}

func TestContainsNodeWithValueHead(t *testing.T) {
   dll := NewDoublyLinkedList[int]()
   node := newNode(1)

   dll.setHead(node)
   if !dll.containsNodeWithValue(1) {
      t.Fail()
   }
}

func TestContainsNodeWithValueTail(t *testing.T) {
   dll := NewDoublyLinkedList[int]()
   node := newNode(1)

   dll.setTail(node)
   if !dll.containsNodeWithValue(1) {
      t.Fail()
   }
}

func TestContainsNodeWithValue(t *testing.T) {
   dll := NewDoublyLinkedList[int]()
   first := newNode(1)
   second := newNode(2)
   third := newNode(3)

   dll.setHead(first)
   dll.insertAfter(first, second)
   dll.insertAfter(second, third)

   if !dll.containsNodeWithValue(2) {
      t.Fail()
   }
}

func newNode(value int) *Node[int] {
   return &Node[int]{
      value: value,
   }
}

func expectingSingleNode(t *testing.T, dll *DoublyLinkedList[int], node *Node[int]) {
   if dll.head != node {
      t.Fail()
   }
   if dll.tail != node {
      t.Fail()
   }
}

func expectingEmpty(t *testing.T, dll *DoublyLinkedList[int]) {
   if dll.head != nil {
      t.Fail()
   }
   if dll.tail != nil {
      t.Fail()
   }
}

func expectingHeadTail(t *testing.T, dll *DoublyLinkedList[int], head *Node[int], tail *Node[int]) {
   if dll.head != head {
      t.Fail()
   }
   if dll.tail != tail {
      t.Fail()
   }
}
