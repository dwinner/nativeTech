use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User
{
   name: String,
   age: u32,
}

async fn create_user(user: web::Json<User>) -> impl Responder
{
   HttpResponse::Ok().json(User {
      name: format!("Hello, {}!", user.name),
      age: user.age,
   })
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
   HttpServer::new(|| App::new().route("/user", web::post().to(create_user)))
      .bind(("127.0.0.1", 8080))?
      .run()
      .await
}
