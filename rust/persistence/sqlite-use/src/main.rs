use rusqlite::{Connection, Result};
use std::env;

fn main() -> Result<()>
{
   let args: Vec<String> = env::args().collect();
   let conn = Connection::open("tasks.db")?;
   create_table(&conn)?;

   if args.len() < 2
   {
      println!("Usage: todo_list [add \"description\" | list | complete id | delete id]");
      return Ok(());
   }

   match args[1].as_str()
   {
      "add" =>
      {
         if args.len() < 3
         {
            println!("Please provide a description");
            return Ok(());
         }

         add_task(&conn, &args[2])?;
         println!("Task added");
      }
      "list" =>
      {
         let tasks = list_tasks(&conn)?;
         for task in tasks
         {
            println!("{}: {} [{}]", task.0, task.1, if task.2 { "done" } else { "not done" });
         }
      }
      "complete" =>
      {
         if args.len() < 3
         {
            println!("Please provide a task id");
            return Ok(());
         }

         let id: i32 = args[2].parse::<i32>().expect("Invalid id");
         complete_task(&conn, id)?;
         println!("Task completed");
      }
      "delete" =>
      {
         if args.len() < 3
         {
            println!("Please provide a task id");
            return Ok(());
         }
         
         let id: i32 = args[2].parse().expect("Invalid id");
         delete_task(&conn, id)?;
         println!("Task deleted");
      }
      _ => println!("Invalid command"),
   }

   Ok(())
}

fn create_table(conn: &Connection) -> Result<()>
{
   conn.execute(
      "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            description TEXT NOT NULL,
            done BOOLEAN NOT NULL DEFAULT 0
        )",
      [],
   )?;
   Ok(())
}

fn add_task(conn: &Connection, description: &str) -> Result<()>
{
   conn.execute("INSERT INTO tasks (description) VALUES (?1)", [description])?;
   Ok(())
}

fn list_tasks(conn: &Connection) -> Result<Vec<(i32, String, bool)>>
{
   let mut stmt = conn.prepare("SELECT id, description, done FROM tasks")?;
   let tasks = stmt
      .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?
      .collect();
   tasks
}

fn complete_task(conn: &Connection, id: i32) -> Result<()>
{
   conn.execute("UPDATE tasks SET done = 1 WHERE id = ?1", [id])?;
   Ok(())
}

fn delete_task(conn: &Connection, id: i32) -> Result<()>
{
   conn.execute("DELETE FROM tasks WHERE id = ?1", [id])?;
   Ok(())
}
