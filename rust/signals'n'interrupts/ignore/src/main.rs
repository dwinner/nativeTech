use libc::{raise, signal};
use libc::{SIGTERM, SIG_DFL, SIG_IGN};

fn main()
{
   unsafe {
      signal(SIGTERM, SIG_IGN);
      raise(SIGTERM);
   }

   println!("ok");

   unsafe {
      signal(SIGTERM, SIG_DFL);
      raise(SIGTERM);
   }

   println!("not ok");
}
