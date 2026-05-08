use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
//use tokio::net::windows::named_pipe::PipeEnd::Server;

async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible>
{
   Ok(Response::new(Body::from("Hello, Hyper!")))
}

#[tokio::main]
async fn main()
{
   let addr = ([127, 0, 0, 1], 8080).into();
   let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

   let server = Server::bind(&addr).serve(make_svc);

   println!("Сервер запущен на http://{}", addr);
   if let Err(e) = server.await
   {
      eprintln!("Ошибка сервера: {}", e);
   }
}
