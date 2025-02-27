fn first_word(str_slice: &str) -> &str
{
   let bytes = str_slice.as_bytes();

   for (idx, &item) in bytes.iter().enumerate()
   {
      if item == b' '
      {
         return &str_slice[0..idx];
      }
   }

   &str_slice[..]
}

fn main()
{
   let my_string = String::from("hello world");

   // `first_word` works on slices of `String`s, whether partial or whole
   let word = first_word(&my_string[0..6]);
   let word = first_word(&my_string[..]);

   // `first_word` also works on references to `String`s, which are equivalent
   // to whole slices of `String`s
   let word = first_word(&my_string);

   let my_string_literal = "hello world";

   // `first_word` works on slices of string literals, whether partial or whole
   let word = first_word(&my_string_literal[0..6]);
   let word = first_word(&my_string_literal[..]);

   // Because string literals *are* string slices already,
   // this works too, without the slice syntax!
   let word = first_word(my_string_literal);
}
