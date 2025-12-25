fn main() {
   // println!("{}", 1 / 0); does not compile

   let _one = 1;
   let _zero = 0;
   // println!("{}", one / zero); does not compile

   let one = 1;
   let _zero = one - 1;
   // println!("{}", one / zero); still doesn't compile

   let defer_one = { || 1 }();
   let defer_zero = { || 0 }();
   println!("{}", defer_one / defer_zero); // panics here
}
