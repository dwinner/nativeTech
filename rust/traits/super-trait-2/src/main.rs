use core::fmt::Debug;

#[derive(Clone, Debug)]
struct CheshireCat
{
   name: String,
}

trait CloneAndDebug: Clone + Debug
{
   fn clone_and_dbg(&self) -> Self
   {
      let cloned = self.clone();
      dbg!(&cloned);
      cloned
   }
}

impl CloneAndDebug for CheshireCat {}

fn main()
{
   let cat = CheshireCat {
      name: "Stephanie".into(),
   };
   let cloned_cat = cat.clone_and_dbg();
   println!("{:?}", cloned_cat);
   println!("{}", cloned_cat.name);
}
