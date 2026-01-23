pub async fn create_router(db_pool: sqlx::Pool<sqlx::Sqlite>) -> axum::Router
{
   use crate::api::{ping, todo_create, todo_delete, todo_list, todo_read, todo_update};
   use axum::{Router, routing::get};
   use tower_http::cors::{Any, CorsLayer};
   use tower_http::trace::TraceLayer;

   Router::new()
      .route("/alive", get(|| async { "ok" }))
      .route("/ready", get(ping))
      .nest(
         "/v1",
         Router::new()
            .route("/todos", get(todo_list).post(todo_create))
            .route("/todos/{id}", get(todo_read).put(todo_update).delete(todo_delete)),
      )
      .with_state(db_pool)
      .layer(CorsLayer::new().allow_methods(Any).allow_origin(Any))
      .layer(TraceLayer::new_for_http())
}
