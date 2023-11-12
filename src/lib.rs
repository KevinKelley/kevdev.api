
pub mod models;
pub mod schema;
pub mod error_handler;

mod handler;
// mod model;
mod response;
mod route;
mod db;

//use error_chain::error_chain;

use anyhow::{Context, Result};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

// error_chain! {
//     foreign_links {
//         Utf8(std::str::Utf8Error);
//         AddrParse(std::net::AddrParseError);
//         Diesel(diesel::result::Error);
//     }
// }
// fn main() -> Result<()> {
//     let parsed = Url::parse("https://httpbin.org/cookies/set?k2=v2&k1=v1")?;



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

////////////////////////////
/// stackoverflow: 
// let mut it = vec![1, 2, 3].into_iter();
// let v : Vec<_> = it.by_ref().take(2).collect();
// println!("v = {:?}", v);
// for x in it {
//     println!("{}", x);
// }
pub fn read_all_todo(conn: &mut PgConnection, offset: u32, limit: u32) -> Vec<Todo> {
    let connection = &mut crate::establish_connection();
    let result = todos
        .filter(completed.eq(false))
        //.skip(offset)     // or...
        //.dropping(offset) // eager
        .limit(limit as i64)
        .select(Todo::as_select())
        .load(connection)
        .expect("Error loading posts");
    return result;
}
/// read a todo by id
pub fn read_todo(conn: &mut PgConnection, tid: i32) -> Result<Option<Todo>, diesel::result::Error> {
// pub fn read_todo(conn: &mut PgConnection, tid: i32) -> Result<Option<Todo>> {
        let connection = &mut crate::establish_connection();
    let todo = todos
        .find(tid)
        .select(Todo::as_select())
        .first(connection)
        .optional(); // This allows for returning an Option<Todo>, otherwise it will throw an error
    return todo;
}
/// find a matching title
pub fn find_todo(conn: &mut PgConnection, ttitle: &str) -> Result<Option<Todo>, diesel::result::Error> {
    let connection = &mut crate::establish_connection();
    let todo = todos
        .filter(title.eq(ttitle))
        .select(Todo::as_select())
        .first(connection)
        .optional(); // This allows for returning an Option<Todo>, otherwise it will throw an error
    return todo;
}
pub fn update_todo(conn: &mut PgConnection, tid: i32, ttitle: &str, tbody: &str, tcompleted: bool) 
    -> Result<Option<Todo>, diesel::result::Error> {
    
    // let updated_row = diesel::update(users.filter(id.eq(1)))
    // .set((name.eq("James"), surname.eq("Bond")))
    // .get_result(connection);
    // assert_eq!(Ok((1, "James".to_string(), "Bond".to_string())), updated_row);
    
    let connection = &mut crate::establish_connection();
    let todo = diesel::update(todos.find(tid))
    .set((title.eq(ttitle), body.eq(tbody), completed.eq(tcompleted)))  
    .returning(Todo::as_returning())
    .get_result(connection)
    .unwrap();
    Ok(Some(todo))
}
pub fn complete_todo(conn: &mut PgConnection, tid: i32) -> Result<Option<Todo>, diesel::result::Error> {
    let connection = &mut crate::establish_connection();
    let todo = diesel::update(todos.find(tid))
    .set(completed.eq(true))  
    .returning(Todo::as_returning())
    .get_result(connection)
    .unwrap();
    Ok(Some(todo))
}
pub fn delete_todo(conn: &mut PgConnection, tid: i32) -> Option<Todo> {
    let connection = &mut crate::establish_connection();
    None
}

