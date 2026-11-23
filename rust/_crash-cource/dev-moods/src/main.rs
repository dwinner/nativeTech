fn developer_mood(caffeine_level: u8) -> &'static str
{
   match caffeine_level
   {
      0 => "I can't work without coffee! ☕",
      1..=2 => "Alright, I can write a few functions.",
      3..=5 => "Productivity mode: ON! 🚀",
      _ => "Too much coffee! I'm rewriting the entire project in Rust! 🔥",
   }
}

fn main()
{
   println!("{}", developer_mood(4));
   // Output: Productivity mode: ON! 🚀
}
