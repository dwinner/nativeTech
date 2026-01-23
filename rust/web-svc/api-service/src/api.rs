use axum::Json;
use axum::extract::{Path, State};
use sqlx::SqlitePool;

use crate::error::Error;
use crate::todo::{CreateTodo, Todo, UpdateTodo};

pub async fn ping(State(db_pool): State<SqlitePool>) -> Result<String, Error>
{
   use sqlx::Connection;

   let mut conn = db_pool.acquire().await?;
   conn.ping().await.map(|_| "ok".to_string()).map_err(Into::into)
}

pub async fn todo_list(State(db_pool): State<SqlitePool>) -> Result<Json<Vec<Todo>>, Error>
{
   Todo::list(db_pool).await.map(Json::from)
}

pub async fn todo_read(State(db_pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Json<Todo>, Error>
{
   Todo::read(db_pool, id).await.map(Json::from)
}

pub async fn todo_create(
   State(db_pool): State<SqlitePool>,
   Json(new_todo): Json<CreateTodo>,
) -> Result<Json<Todo>, Error>
{
   Todo::create(db_pool, new_todo).await.map(Json::from)
}

pub async fn todo_update(
   State(db_pool): State<SqlitePool>,
   Path(id): Path<i64>,
   Json(updated_todo): Json<UpdateTodo>,
) -> Result<Json<Todo>, Error>
{
   Todo::update(db_pool, id, updated_todo).await.map(Json::from)
}

pub async fn todo_delete(State(db_pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<(), Error>
{
   Todo::delete(db_pool, id).await
}
