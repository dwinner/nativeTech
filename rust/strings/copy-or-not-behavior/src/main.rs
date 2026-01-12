fn lowercased(a_str: String) -> String
{
   a_str.to_lowercase()
}

fn lowercased_ascii(mut a_str: String) -> String
{
   a_str.make_ascii_lowercase();
   a_str
}

fn main()
{
   let str1 = "HeLlO";
   let str2 = "CoMpUtErS";
   println!("lowercased('{}') -> '{}'", str1, lowercased(str1.to_owned()));
   println!(
      "lowercased_ascii('{}`) -> '{}'",
      str2,
      lowercased_ascii(str2.to_owned())
   );
}
