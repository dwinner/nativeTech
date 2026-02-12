#![warn(rust_2018_idioms)]
#![allow(elided_lifetimes_in_paths)]

/// A first-in, first-out queue of characters.
pub struct Queue
{
   older: Vec<char>,   // older elements, eldest last.
   younger: Vec<char>, // younger elements, youngest last.
}

impl Queue
{
   /// Push a character onto the back of a queue.
   pub fn push(&mut self, a_char: char)
   {
      self.younger.push(a_char);
   }

   /// Pop a character off the front of a queue. Return `Some(c)` if there
   /// was a character to pop, or `None` if the queue was empty.
   pub fn pop(&mut self) -> Option<char>
   {
      if self.older.is_empty()
      {
         if self.younger.is_empty()
         {
            return None;
         }

         // Bring the elements in younger over to older, and put them in
         // the promised order.
         use std::mem::swap;
         swap(&mut self.older, &mut self.younger);
         self.older.reverse();
      }

      // Now older is guaranteed to have something. Vec's pop method
      // already returns an Option, so we're set.
      self.older.pop()
   }
}

#[test]
fn test_push_pop()
{
   let mut queue = Queue {
      older: Vec::new(),
      younger: Vec::new(),
   };

   queue.push('0');
   queue.push('1');
   assert_eq!(queue.pop(), Some('0'));

   queue.push('∞');
   assert_eq!(queue.pop(), Some('1'));
   assert_eq!(queue.pop(), Some('∞'));
   assert_eq!(queue.pop(), None);

   (&mut queue).push('0');
   (&mut queue).push('1');
   assert_eq!(queue.pop(), Some('0'));
   assert_eq!(queue.pop(), Some('1'));
}

impl Queue
{
   pub fn is_empty(&self) -> bool
   {
      self.older.is_empty() && self.younger.is_empty()
   }
}

#[test]
fn test_is_empty()
{
   let mut queue = Queue {
      older: Vec::new(),
      younger: Vec::new(),
   };

   assert!(queue.is_empty());
   queue.push('☉');
   assert!(!queue.is_empty());
   queue.pop();
   assert!(queue.is_empty());
}

impl Queue
{
   pub fn split(self) -> (Vec<char>, Vec<char>)
   {
      (self.older, self.younger)
   }
}

#[test]
fn test_split()
{
   let mut queue = Queue {
      older: Vec::new(),
      younger: Vec::new(),
   };

   queue.push('P');
   queue.push('D');
   assert_eq!(queue.pop(), Some('P'));
   queue.push('X');

   let (older, younger) = queue.split();
   // q is now uninitialized.
   assert_eq!(older, vec!['D']);
   assert_eq!(younger, vec!['X']);
}

impl Queue
{
   pub fn new() -> Queue
   {
      Queue {
         older: Vec::new(),
         younger: Vec::new(),
      }
   }
}

#[test]
fn test_new()
{
   let mut queue = Queue::new();

   queue.push('*');
   queue.push('1');
   assert_eq!(queue.pop(), Some('*'));

   queue.push('∞');
   assert_eq!(queue.pop(), Some('1'));
   assert_eq!(queue.pop(), Some('∞'));
   assert_eq!(queue.pop(), None);
}
