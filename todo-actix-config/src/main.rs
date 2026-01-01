mod models;
mod config;
mod handler;
mod db;

use crate::config::AppConfig;
// use crate::models::Status;
use actix_web::{App, HttpServer, web};
use std::io;
use tokio_postgres::NoTls;
use crate::handler::status; 


#[actix_web::main] // Use actix_web::main for consistency with actix-web 4.x
async fn main() -> io::Result<()> {
   // 1. Load .env file into the system environment
    dotenv::dotenv().ok(); 

    // 2. Now from_env() will find them in the system env
    let config = AppConfig::from_env().expect("Failed to load configuration");
    println!("Starting server at http://{}:{}", config.server.host, config.server.port);

    // create a connection pool; supply the runtime and TLS mode
    let pool = config
        .pg
        .create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)
        .unwrap();


    // 2. Start the server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(status))
            .route("/todo_lists{_:/?}", web::get().to(handler::get_todo_lists))
    })
    // Passing a tuple (host, port) is cleaner than format!() strings
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}