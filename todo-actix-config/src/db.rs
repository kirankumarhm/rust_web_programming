// use crate::models::{TodoList};
// use deadpool_postgres::Client;
// use tokio_postgres::Error;
use tokio_pg_mapper::FromTokioPostgresRow;
use crate::models::{ TodoList};
use deadpool_postgres::Client;
use tokio_postgres::Error;

pub async fn get_todo_lists(db_client: &Client) -> Result<Vec<TodoList>, Error> {
    let stmt = db_client.prepare("SELECT id, title FROM todo_lists").await?;

    let rows = db_client.query(&stmt, &[]).await?;

    let todo_lists = rows
        .iter()
        .map(|row| TodoList::from_row_ref(row).expect("Failed to map row to TodoList"))
        .collect::<Vec<TodoList>>();

    Ok(todo_lists)
}
