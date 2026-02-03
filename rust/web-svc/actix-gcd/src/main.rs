use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main()
{
   let server = HttpServer::new(|| {
      App::new()
         .route("/", web::get().to(get_index))
         .route("/gcd", web::post().to(post_gcd))
   });

   println!("Serving on http://localhost:3000...");
   server
      .bind("127.0.0.1:3000")
      .expect("error binding server to address")
      .run()
      .await
      .expect("error running server");
}

async fn get_index() -> HttpResponse
{
   HttpResponse::Ok().content_type("text/html").body(
      r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                <input type="text" name="num1"/>
                <input type="text" name="num2"/>
                <button type="submit">Compute GCD</button>
                </form>
            "#,
   )
}

use serde::Deserialize;
#[derive(Deserialize)]
struct GcdParameters
{
   num1: u64,
   num2: u64,
}

async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse
{
   if form.num1 == 0 || form.num2 == 0
   {
      return HttpResponse::BadRequest()
         .content_type("text/html")
         .body("Computing the GCD with zero is boring.");
   }

   let response = format!(
      "The greatest common divisor of the numbers {} and {} is <b>{}</b>\n",
      form.num1,
      form.num2,
      gcd(form.num1, form.num2)
   );

   HttpResponse::Ok().content_type("text/html").body(response)
}

fn gcd(mut num1: u64, mut num2: u64) -> u64
{
   assert!(num1 != 0 && num2 != 0);

   while num2 != 0
   {
      if num2 < num1
      {
         let tmp = num2;
         num2 = num1;
         num1 = tmp;
      }

      num2 = num2 % num1;
   }

   num1
}
