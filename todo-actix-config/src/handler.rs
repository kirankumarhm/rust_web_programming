
use actix_web::{Responder, web, HttpResponse};
use crate::models::Status;


pub async fn status() -> impl Responder {
    web::Json(Status { status: "UP".to_string() })
}


pub async fn get_todo_lists(db_pool: web::Data<deadpool_postgres::Pool>) -> impl Responder {
    use crate::db::get_todo_lists;

    // Acquire a client from the pool
    match db_pool.get().await {
        Ok(client) => match get_todo_lists(&client).await {
            Ok(lists) => HttpResponse::Ok().json(lists),
            Err(e) => HttpResponse::InternalServerError().body(format!("Error fetching todo lists: {}", e)),
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to get DB client from pool: {}", e)),
    }
}


