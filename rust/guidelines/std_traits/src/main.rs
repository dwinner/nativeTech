fn main()
{
   value_moved();
   value_copied();
   use_default();
}

#[derive(Debug, Clone)]
struct KeyId(u32);

#[derive(Debug, Clone, Copy)]
struct KeyIdV2(u32);

#[derive(Default)]
enum IceCreamFlavor
{
   Chocolate,
   Strawberry,
   #[default]
   Vanilla,
}

#[derive(Default, Debug)]
struct Color
{
   red: u8,
   green: u8,
   blue: u8,
   alpha: u8,
}

fn use_default()
{
   let c = Color {
      red: 128,
      ..Default::default()
   };

   println!("{c:?}");
}

fn value_moved()
{
   let k = KeyId(42);
   let k2 = k; // value moved
   //println!("k={k:?}");
   println!("k={k2:?}");
}

fn value_copied()
{
   let k = KeyIdV2(42);
   let k2 = k; // value copied bitwise
   println!("k={k:?}");
   println!("k={k2:?}");
}
