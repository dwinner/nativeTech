use std::cell::RefCell;
use std::thread;

thread_local! {
    static MY_NOTEBOOK: RefCell<i32> = RefCell::new(0);
}

fn main()
{
   let handles: Vec<_> = (0..3)
      .map(|_| {
         thread::spawn(|| {
            MY_NOTEBOOK.with(|note| {
               *note.borrow_mut() += 1;
               println!("My notes: {}", *note.borrow());
            });
         })
      })
      .collect();

   for handle in handles
   {
      handle.join().unwrap();
   }
}
