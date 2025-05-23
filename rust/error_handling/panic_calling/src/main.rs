use guessing_game::Guess;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

mod guessing_game;

fn main()
{
   println!("Guess the number!");

   let secret_number = rand::rng().random_range(1..=100);

   loop
   {
      println!("Please input your guess.");

      let mut guess = String::new();

      io::stdin().read_line(&mut guess).expect("Failed to read line");

      let guess: i32 = match guess.trim().parse()
      {
         Ok(num) => num,
         Err(_) => continue,
      };

      let guess = Guess::new(guess);

      match guess.get_value().cmp(&secret_number)
      {
         Ordering::Less => println!("Too small!"),
         Ordering::Greater => println!("Too big!"),
         Ordering::Equal =>
         {
            println!("You win!");
            break;
         }
      }
   }
}
