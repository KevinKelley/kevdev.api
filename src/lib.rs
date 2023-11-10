
pub mod models;
pub mod schema;
pub mod error_handler;

mod handler;
mod model;
mod response;
mod route;
mod db;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

/////////////////////////////////
/// db access api fns
/// 

use self::models::{NewTodo, Todo};

pub fn create_todo(conn: &mut PgConnection, title: &str, body: &str) -> Todo {
    use crate::schema::todos;

    let new_todo = NewTodo { title, body };

    diesel::insert_into(todos::table)
        .values(&new_todo)
        .returning(Todo::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}