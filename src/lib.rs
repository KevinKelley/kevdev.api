
pub mod models;
pub mod schema;
pub mod error_handler;

mod handler;
// mod model;
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

use crate::error_handler::{CustomError};

use crate::models::{NewTodo, Todo};
use diesel::prelude::*;
use crate::schema::todos::dsl::*;

pub fn create_todo(conn: &mut PgConnection, ttitle: &str, tbody: &str) -> Todo {
    let connection = &mut crate::establish_connection();

    use crate::schema::todos;

    let new_todo = NewTodo { title:ttitle, body:tbody };

    diesel::insert_into(todos::table)
        .values(&new_todo)
        .returning(Todo::as_returning())
        .get_result(connection)
        .expect("Error saving new post")
}

pub fn read_all_todo(conn: &mut PgConnection, skip: u32, limit: u32) -> Vec<Todo> {
    let connection = &mut crate::establish_connection();
    let result = todos
        .filter(completed.eq(false))
        .skip(skip)
        .limit(limit)
        .select(Todo::as_select())
        .load(connection)
        .expect("Error loading posts");
    return result;
}
pub fn read_todo(conn: &mut PgConnection, tid: i32) -> Result<Option<Todo>, CustomError> {
    let connection = &mut crate::establish_connection();
    let todo = todos
        .find(tid)
        .select(Todo::as_select())
        .first(connection)
        .optional(); // This allows for returning an Option<Todo>, otherwise it will throw an error
    return todo;
}
pub fn find_todo(conn: &mut PgConnection, ttitle: &str) -> Option<Todo> {
    let connection = &mut crate::establish_connection();
    None
}
pub fn update_todo(conn: &mut PgConnection, tid: i32, ttitle: &str, tbody: &str, tcompleted: bool) -> Todo {
    let connection = &mut crate::establish_connection();
    create_empty_todo()
}
pub fn complete_todo(conn: &mut PgConnection, tid: i32) -> Todo {
    let connection = &mut crate::establish_connection();
    let todo = diesel::update(todos.find(tid))
    .set(completed.eq(true))   // ?????
    .returning(Todo::as_returning())
    .get_result(connection)
    .unwrap();
    todo
}
pub fn delete_todo(conn: &mut PgConnection, tid: i32) -> Option<Todo> {
    let connection = &mut crate::establish_connection();
    None
}


fn create_empty_todo() -> Todo {
    Todo { id:0, title:"".to_string(), body:"".to_string(), completed:false }
}
