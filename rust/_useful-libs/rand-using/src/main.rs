// Declare basic functions for pseudo-random number generators.
use rand::prelude::*;

fn main()
{
   // Create a pseudo-Random Number Generator for the current thread
   let mut rng = rand::rng();

   // Print an integer number
   // between 0 (included) and 20 (excluded).
   println!("{}", rng.random_range(0..=20));

   // Print a floating-point number
   // between 0 (included) and 1 (excluded).
   println!("{}", rng.random::<f32>());

   // Generate a Boolean.
   println!("{}", if rng.random_bool(0.5f64) { "Heads" } else { "Tails" });
}
