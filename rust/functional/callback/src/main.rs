fn callback_fn<F>(a_func: F)
where
   F: Fn() -> (),
{
   a_func();
}

fn main()
{
   let my_callback = || println!("I have been called back");
   callback_fn(my_callback);
}
