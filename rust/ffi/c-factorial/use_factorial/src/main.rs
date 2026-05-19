#[link(name = "factorial")]
unsafe extern "C" {
   fn factorial(n: u32) -> u64;
}

// Безопасная обертка
fn safe_factorial(n: u32) -> Option<u64> {
   if n > 20 {
      // Предотвращаем переполнение
      return None;
   }

   Some(unsafe { factorial(n) })
}

fn main() {
   let n = 5;
   match safe_factorial(n) {
      Some(result) => println!("Factorial of {} is {}", n, result),
      None => println!("Input too large"),
   }
}
