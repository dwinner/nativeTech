// Define a struct named `Order` with the following fields:
// - `price`, an unsigned integer
// - `quantity`, an unsigned integer
//
// It should also have a method named `is_available` that returns a `true` if the quantity is
// greater than 0, otherwise `false`.

struct Order
{
   price: u32,
   quantity: u32,
}

impl Order
{
   fn is_available(&self) -> bool
   {
      if self.quantity > 0 { true } else { false }
   }
}

fn main()
{
   let order = Order {
      price: 100,
      quantity: 10,
   };
   assert!(order.is_available());

   let order = Order {
      price: 100,
      quantity: 0,
   };
   assert!(!order.is_available());
}
