use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::Bson::ObjectId;
use mongodb::{Client, Collection};

#[tokio::main]
async fn main() -> mongodb::error::Result<()>
{
   let client = Client::with_uri_str("mongodb://localhost:27017/").await?;
   let db = client.database("tasks_db");
   let collection: Collection<mongodb::bson::Document> = db.collection("tasks");

   // Добавляем задачи
   add_task(&collection, "Купить молоко").await?;
   add_task(&collection, "Позвонить другу").await?;

   // Получаем и выводим все задачи
   let tasks = get_tasks(&collection).await?;
   for task in tasks
   {
      println!("Задача: {:?}", task);
   }

   // Предположим, что у нас есть _id задачи, например, "507f1f77bcf86cd799439011"
   let task_id = "507f1f77bcf86cd799439011";
   complete_task(&collection, task_id).await?;

   // Удаляем задачу по _id
   delete_task(&collection, task_id).await?;

   Ok(())
}

async fn add_task(collection: &Collection<mongodb::bson::Document>, description: &str) -> mongodb::error::Result<()>
{
   let doc = doc! { "description": description, "completed": false };
   collection.insert_one(doc, None).await?;
   Ok(())
}

async fn get_tasks(
   collection: &Collection<mongodb::bson::Document>,
) -> mongodb::error::Result<Vec<mongodb::bson::Document>>
{
   let mut cursor = collection.find(None).await?;
   let mut tasks = Vec::new();
   while let Some(result) = cursor.next().await
   {
      tasks.push(result?);
   }

   Ok(tasks)
}

async fn complete_task(collection: &Collection<mongodb::bson::Document>, id: &str) -> mongodb::error::Result<()>
{
   let obj_id = ObjectId::with_string(id)?;
   let filter = doc! { "_id": obj_id };
   let update = doc! { "$set": { "completed": true } };
   collection.update_one(filter, update/*, None*/).await?;
   Ok(())
}

async fn delete_task(collection: &Collection<mongodb::bson::Document>, id: &str) -> mongodb::error::Result<()>
{
   let obj_id = ObjectId::with_string(id)?;
   let filter = doc! { "_id": obj_id };
   collection.delete_one(filter /* , None */).await?;

   Ok(())
}
