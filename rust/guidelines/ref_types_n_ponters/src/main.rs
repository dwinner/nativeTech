fn main()
{
   let pt = Point { x: 1, y: 2 };
   //let x=0u64;
   //let ref_x=&x;
   let ref_pt = &pt;

   // on the heap
   let box_pt = Box::new(Point { x: 10, y: 20 });

   // since Box<T> implements Deref, the following is possible
   show(ref_pt);
   show(&box_pt);
}

pub struct Point
{
   pub x: u32,
   pub y: u32,
}

fn show(pt: &Point)
{
   println!("({}, {})", pt.x, pt.y);
}
