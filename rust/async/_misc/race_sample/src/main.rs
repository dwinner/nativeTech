extern crate trpl;

use std::time::Duration;

fn main()
{
   trpl::run(async {
      let slow = async {
         println!("'slow' started.");
         trpl::sleep(Duration::from_millis(100)).await;
         println!("'slow' finished.");
      };

      let fast = async {
         println!("'fast' started.");
         trpl::sleep(Duration::from_millis(50)).await;
         println!("'fast' finished.");
      };

      trpl::race(slow, fast).await;
   });
}
