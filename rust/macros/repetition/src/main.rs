macro_rules! print_all {
    ($($arg:expr),*) => {
        $(
            println!("{}", $arg);
        )*
    };
}

fn main()
{
   print_all!(1, "hello", 3.14);
}
