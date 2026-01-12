mod build_data_structures;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

fn main()
{
   // It's Ok to have several immutable references
   // until you introduce a mutable one
   let mut item = Item { contents: 42 };
   let r = &/*mut*/ item;
   let contents = item.contents;
   println!("reference to item is {:?}", r);

   extend_lifetime_demo();
   split_vars_by_lifetime();
}

fn extend_lifetime_demo()
{
   let haystack = format!("{} to search", "Text");
   let found = find(&haystack, "ex");
   if let Some(text) = found
   {
      println!("Found '{text}'");
   }
}

fn split_vars_by_lifetime()
{
   let x: Option<Rc<RefCell<Item>>> = Some(Rc::new(RefCell::new(Item { contents: 42 })));
   let x1: Option<&Rc<RefCell<Item>>> = x.as_ref();
   let x2: Option<std::cell::Ref<Item>> = x1.map(|r| r.borrow());

   /*let x3: Option<&Item> = x2.map(|r| r.deref());
   check_item(x3);*/

   match x2
   {
      None => check_item(None),
      Some(r) =>
      {
         let x3: &Item = r.deref();
         check_item(Some(x3));
      }
   }
}

/// Some data structure, using in the code
#[derive(Debug)]
pub struct Item
{
   pub contents: i64,
}

/// Replace `item` with `val` with returning previous content
pub fn replace(item: &mut Option<Item>, val: Item) -> Option<Item>
{
   // Alt: std::mem::replace(item,Some(val))
   item.replace(val) // return previous value
}

/// If `needle` is in `haystack`,
/// return a slice containing it
pub fn find<'a, 'b>(haystack: &'a str, needle: &'b str) -> Option<&'a str>
{
   haystack.find(needle).map(|i| &haystack[i..i + needle.len()])
}

pub fn check_item(item: Option<&Item>)
{
   todo!()
}
