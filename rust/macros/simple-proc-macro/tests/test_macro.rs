#[cfg(test)]
mod tests
{
   #[test]
   fn print_hello_world()
   {
      use simple_proc_macro::say_hello_world;
      say_hello_world!();
   }
}
