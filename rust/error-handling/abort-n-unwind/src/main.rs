fn _drink(beverage: &str)
{
   // You shouldn't drink too much sugary beverages.
   if beverage == "lemonade"
   {
      if cfg!(panic = "abort")
      {
         println!("This is not your party. Run!!!!");
      }
      else
      {
         println!("Spit it out!!!!");
      }
   }
   else
   {
      println!("Some refreshing {} is all I need.", beverage);
   }
}

#[cfg(panic = "unwind")]
fn ah()
{
   println!("Spit it out!!!!");
}

#[cfg(not(panic = "unwind"))]
fn ah()
{
   println!("This is not your party. Run!!!!");
}

fn drink_2(beverage: &str)
{
   if beverage == "lemonade"
   {
      ah();
   }
   else
   {
      println!("Some refreshing {} is all I need.", beverage);
   }
}

fn main()
{
   drink_2("water");
   drink_2("lemonade");
}
