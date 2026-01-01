use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;


#[derive(Serialize, Deserialize)]
pub struct Status {
    pub status: String
}


#[derive(PostgresMapper, Serialize, Deserialize)]
#[pg_mapper(table = "todo_lists")]
pub struct TodoList {
    pub id: i32,
    pub title: String
}

// #[derive(PostgresMapper, Serialize, Deserialize)]
// #[pg_mapper(table = "todo_items")]
// pub struct TodoItem {
//     pub id: i32,
//     pub title: String,
//     pub checked: bool,
//     pub list_id: i32
// }