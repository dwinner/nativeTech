use std::collections::HashMap;
use std::io::Read;

fn main()
{
   let args: Vec<String> = std::env::args().collect();
   if args.len() < 3
   {
      panic!("Too less arguments passed...");
   }

   let action = args[1].clone();
   let task = args[2].clone();

   let mut todo = ToDo::new().expect("Error in ToDo list creation");

   if action == "add"
   {
      todo.insert(task);
      match todo.save()
      {
         Ok(_) => println!("Task added"),
         Err(e) => println!("Error : {}", e),
      }
   }
   else if action == "start"
   {
      match todo.start(&task)
      {
         None => println!("'{}' not present in ToDo list", task),
         Some(_) => match todo.save()
         {
            Ok(_) => println!("Task started"),
            Err(e) => println!("Error : {}", e),
         },
      }
   }
   else if action == "done"
   {
      match todo.done(&task)
      {
         None => println!("'{}' not present in ToDo list", task),
         Some(_) => match todo.save()
         {
            Ok(_) => println!("Task Done"),
            Err(e) => println!("Error : {}", e),
         },
      }
   }
}

struct ToDo
{
   map: HashMap<String, String>,
}

impl ToDo
{
   fn new() -> Result<ToDo, std::io::Error>
   {
      const DB_NAME: &'static str = "todo.db";
      let mut todo_file = std::fs::OpenOptions::new()
         .write(true)
         .create(true)
         .read(true)
         .open(DB_NAME)?;
      let mut content = String::new();
      todo_file.read_to_string(&mut content)?;
      let map: HashMap<String, String> = content
         .lines()
         .map(|line| line.split(" : ").collect::<Vec<&str>>())
         .map(|vec_item| (vec_item[0], vec_item[1]))
         .map(|(key, val)| (String::from(key), String::from(val)))
         .collect();

      Ok(ToDo { map })
   }

   fn insert(&mut self, key: String)
   {
      self.map.insert(key, String::from("To Do"));
   }

   fn save(self) -> Result<(), std::io::Error>
   {
      const DB_NAME: &'static str = "todo.db";
      let mut content = String::new();
      for (key, val) in self.map
      {
         let record = format!("{} : {}\n", key, val);
         content.push_str(&record);
      }

      std::fs::write(DB_NAME, content)
   }

   fn start(&mut self, key: &String) -> Option<()>
   {
      match self.map.get_mut(key)
      {
         None => None,
         Some(val) => Some(*val = String::from("In progress")),
      }
   }

   fn done(&mut self, key: &String) -> Option<()>
   {
      match self.map.get_mut(key)
      {
         None => None,
         Some(val) => Some(*val = String::from("Done")),
      }
   }
}
