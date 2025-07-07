use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;

fn main()
{
   let args: Vec<String> = std::env::args().collect();
   if args.len() != 3
   {
      panic!("Too less arguments passed...");
   }

   let action = args[1].clone();
   let task = args[2].clone();
   let mut todo = ToDo::new().expect("Error in ToDo list creation");
   let task_state = TaskState::from_str(&action);
   match task_state
   {
      Ok(state) => match state
      {
         TaskState::Added =>
         {
            todo.insert(task);
            match todo.save()
            {
               Ok(_) => println!("Task added"),
               Err(e) => println!("Error : {}", e),
            }
         }
         TaskState::Started => match todo.start(&task)
         {
            None => println!("'{}' not present in ToDo list", task),
            Some(_) => match todo.save()
            {
               Ok(_) => println!("Task started"),
               Err(e) => println!("Error : {}", e),
            },
         },
         TaskState::Done => match todo.done(&task)
         {
            None => println!("'{}' not present in ToDo list", task),
            Some(_) => match todo.save()
            {
               Ok(_) => println!("Task Done"),
               Err(e) => println!("Error : {}", e),
            },
         },
      },
      Err(_) =>
      {
         panic!("Unsupported action {}", action);
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
      const DB_NAME: &str = "todo.db";
      let mut todo_file = std::fs::OpenOptions::new()
         .write(true)
         .create(true)
         .truncate(true)
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
      const DB_NAME: &str = "todo.db";
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
         Some(val) =>
         {
            *val = String::from("In progress");
            Some(())
         }
      }
   }

   fn done(&mut self, key: &String) -> Option<()>
   {
      match self.map.get_mut(key)
      {
         None => None,
         Some(val) =>
         {
            *val = String::from("Done");
            self.remove(key);
            Some(())
         }
      }
   }

   fn remove(&mut self, key: &String) -> Option<()>
   {
      match self.map.remove_entry(key)
      {
         None => None,
         Some(_) => Some(()),
      }
   }
}

#[derive(Debug, PartialEq)]
enum TaskState
{
   Added,
   Started,
   Done,
}

impl FromStr for TaskState
{
   type Err = ();

   fn from_str(input_source: &str) -> Result<TaskState, Self::Err>
   {
      match input_source
      {
         "add" => Ok(TaskState::Added),
         "start" => Ok(TaskState::Started),
         "done" => Ok(TaskState::Done),
         _ => Err(()),
      }
   }
}
