use my_proc_macro::log_call;
use my_proc_macro::Hello;

#[derive(Hello)]
struct MyStruct;

#[log_call]
fn do_work()
{
   println!("Working...");
}

fn main()
{
   let my_struct = MyStruct {};
   my_struct.say_hello(); // Вывод: Hello from MyStruct!
   do_work();
}
