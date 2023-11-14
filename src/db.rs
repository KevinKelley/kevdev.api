use crate::error_handler::CustomError;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::prelude::*;
use lazy_static::lazy_static;
use r2d2;
use dotenvy::dotenv;
use std::env;

use std::sync::Arc;
use tokio::sync::Mutex;


type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;
// pub type DbConnection = Arc<Mutex<r2d2::PooledConnection<ConnectionManager<PgConnection>>>>;
// pub type DB = crate::db::DbConnection;

// embed_migrations!();

lazy_static! {
    static ref POOL: Pool = {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

pub fn init() {
    lazy_static::initialize(&POOL);
    let conn = connection().expect("Failed to get db connection");
    // embedded_migrations::run(&conn).unwrap();
}

pub fn connection() -> Result<DbConnection, CustomError> {
    POOL.get()
        //.map_err(|e| CustomError::new(500, format!("Failed getting db connection: {}", e)))
        .map_err(|e| CustomError{code:500, message:format!("Failed getting db connection: {}", e)})
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// - let connection = PgConnection::establish_connection("…")?;
// - let result = some_query.load(&connection)?;
// + let mut connection = PgConnection::establish_connection("…")?;
// + let result = some_query.load(&mut connection)?;


// pub fn todo_db() -> DB {
//     Arc::new(Mutex::new(connection()))
// }

// use std::sync::Arc;
// use tokio::sync::Mutex;
// pub type DB = Arc<Mutex<Vec<Todo>>>;

// pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;


// #[allow(non_snake_case)]
// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct Todo {
//     pub id: Option<String>,
//     pub title: String,
//     pub content: String,
//     pub completed: Option<bool>,
//     pub createdAt: Option<DateTime<Utc>>,
//     pub updatedAt: Option<DateTime<Utc>>,
// }

// pub type DB = Arc<Mutex<Vec<Todo>>>;

// pub fn todo_db() -> DB {
//     Arc::new(Mutex::new(Vec::new()))
// }

// let engine = "mysql";
// let username = "root";
// let password = "123456";
// let host = "localhost";
// let port = 3306;
// let db = "test";
// let url = format!(
//     "{}://{}:{}@{}:{}/{}",
//     engine, username, password, host, port, db
// );
// let connection = MysqlConnection::establish(&url)
//     .expect(&format!("Failed to connect database:{}-{}", engine, db));
// Database { connection }




// use anyhow::{Context, Result};
//use error_chain::error_chain;
// error_chain! {
//     foreign_links {
//         Utf8(std::str::Utf8Error);
//         AddrParse(std::net::AddrParseError);
//         Diesel(diesel::result::Error);
//     }
// }
// fn main() -> Result<()> {
//     let parsed = Url::parse("https://httpbin.org/cookies/set?k2=v2&k1=v1")?;


/////////////////////////////////
/// db access api fns
/// 

// use crate::error_handler::{CustomError};

use crate::models::{NewTodo, Todo};
use diesel::prelude::*;
use crate::schema::todos::dsl::*;

pub fn create_todo(/*conn: &mut PgConnection,*/ ttitle: &str, tbody: &str) -> Todo {
    let connection = &mut establish_connection();

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
pub fn read_all_todo(/*conn: &mut PgConnection,*/ offset: u32, limit: u32) -> Vec<Todo> {
    let connection = &mut establish_connection();
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
pub fn read_todo(/*conn: &mut PgConnection,*/ tid: i32) -> Result<Option<Todo>, diesel::result::Error> {
// pub fn read_todo(/*conn: &mut PgConnection,*/ tid: i32) -> Result<Option<Todo>> {
        let connection = &mut establish_connection();
    let todo = todos
        .find(tid)
        .select(Todo::as_select())
        .first(connection)
        .optional(); // This allows for returning an Option<Todo>, otherwise it will throw an error
    return todo;
}
/// find a matching title
pub fn find_todo(/*conn: &mut PgConnection,*/ ttitle: &str) -> Result<Option<Todo>, diesel::result::Error> {
    let connection = &mut establish_connection();
    let todo = todos
        .filter(title.eq(ttitle))
        .select(Todo::as_select())
        .first(connection)
        .optional(); // This allows for returning an Option<Todo>, otherwise it will throw an error
    return todo;
}
pub fn update_todo(/*conn: &mut PgConnection,*/ tid: i32, ttitle: &str, tbody: &str, tcompleted: bool) 
    -> Result<Option<Todo>, diesel::result::Error> {
    
    // let updated_row = diesel::update(users.filter(id.eq(1)))
    // .set((name.eq("James"), surname.eq("Bond")))
    // .get_result(connection);
    // assert_eq!(Ok((1, "James".to_string(), "Bond".to_string())), updated_row);
    
    let connection = &mut establish_connection();
    let todo = diesel::update(todos.find(tid))
    .set((title.eq(ttitle), body.eq(tbody), completed.eq(tcompleted)))  
    .returning(Todo::as_returning())
    .get_result(connection)
    .unwrap();
    Ok(Some(todo))
}
pub fn complete_todo(/*conn: &mut PgConnection,*/ tid: i32) -> Result<Option<Todo>, diesel::result::Error> {
    let connection = &mut establish_connection();
    let todo = diesel::update(todos.find(tid))
    .set(completed.eq(true))  
    .returning(Todo::as_returning())
    .get_result(connection)
    .unwrap();
    Ok(Some(todo))
}
pub fn delete_todo(/*conn: &mut PgConnection,*/ tid: i32) -> Result<Option<Todo>, diesel::result::Error> {
    let connection = &mut establish_connection();
    let todo = diesel::delete(todos.find(tid))
    .returning(Todo::as_returning())
    .get_result(connection)
    .unwrap();
    Ok(Some(todo))

    // let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
    //     .execute(connection)
    //     .expect("Error deleting posts");
}

