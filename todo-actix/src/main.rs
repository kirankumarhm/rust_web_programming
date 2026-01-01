mod models;


use crate::models::Status;
use actix_web::{App, HttpServer, Responder, web};
use std::io;

async fn status() -> impl Responder {
    // return http response with json body
    web::Json(Status { status: "UP".to_string() })
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
   
   println!("Starting server at http://127.0.0.1:8080");

   HttpServer::new(||  {
    App::new()
       .route("/", web::get().to(status))
   })
   .bind("127.0.0.1:8080")?
   .run()
   .await
}
