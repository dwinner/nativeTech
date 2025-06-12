static ANSWER: Item = Item { contents: 42 };

fn main()
{
   {
      let item1 = Item { contents: 1 }; // `item1` created
      let item2 = Item { contents: 2 }; // `item2` created
      println!("item1={item1:?}, item2={item2:?}");
      consuming_fn(item2); // `item2` is moved
   } // `item1` is dropped

   {
      // `s` owns `String`
      let mut s: String = "Hello, worlds".to_string();

      // creates mutable reference to string
      let greeting = &mut s[..5];
      greeting.make_ascii_uppercase();
      // ... after that `greeting` isn't used anymore

      // creating of immutable reference to 'string' is accepted,
      // in spite of mutable reference is in scope
      let r: &str = &s;
      println!("s = '{}'", r); // s = 'HELLO, world'
   } // `greeting` is dropped there

   {
      let haystack = b"123456789"; // starting lifetime `a`
      let found = {
         let needle = b"234"; // starting lifetime `b`
         find(haystack, needle)
      }; // end of lifetime `b`

      // `found` is used within `a`, not `b`
      println!("found={:?}", found);
   } // end of lifetime `a`

   println!("{:?}", the_answer());

   {
      let boxed = Box::new(Item { contents: 12 });

      // isn't compiled
      //let r: &'static Item=&boxed;

      // `leak()` gets `Box<T>` and returns `&mut T`
      let r: &'static Item = Box::leak(boxed);
      println!("static item is {:?}", r);
   }
}

struct ReferenceHolder<'a>
{
   index: usize,
   item: &'a Item,
}

// anonymous lifetime
fn find_one_item(items: &[Item]) -> ReferenceHolder<'_>
{
   todo!()
}

fn consuming_fn(item: Item)
{
   todo!()
}

fn find<'a, 'b>(haystack: &'a [u8], needle: &'b [u8]) -> Option<&'a [u8]>
{
   todo!()
}

fn the_answer() -> &'static Item
{
   &ANSWER
}

#[derive(Debug)]
pub struct Item
{
   contents: u32,
}
