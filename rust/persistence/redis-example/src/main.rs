extern crate redis;
use redis::TypedCommands;

fn main() -> redis::RedisResult<()>
{
   let client = redis::Client::open("redis://man:man@localhost:6380/0")?;
   let mut conn = client.get_connection()?;

   conn.set("aKey", "a string")?;
   conn.set("anotherKey", 4567)?;
   conn.set(45, 12345)?;

   let key = conn.get("aKey")?;
   let another_key = conn.get("anotherKey")?;
   let a45 = conn.get("45")?;
   /*let a40 = conn.get("40")?;
   let a_bool = conn.get("40")?;*/

   println!(
      "{}, {}, {}.",
      key.unwrap(),
      another_key.unwrap(),
      a45.unwrap(),
      /*a40,
      a_bool.unwrap()*/
   );

   Ok(())
}
