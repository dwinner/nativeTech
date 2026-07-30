use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
   static ref DICTIONARY: HashMap<u32, &'static str> = {
      let mut map = HashMap::new();
      map.insert(11, "foo");
      map.insert(12, "bar");
      println!("Initialized");

      map
   };
}

fn main()
{
   println!("Started");
   println!("DICTIONARY contains {:?}", *DICTIONARY);
   println!("DICTIONARY contains {:?}", *DICTIONARY);
}
