use self::models::Todo;
use diesel::prelude::*;
use todo_api::*;
use std::env::args;

fn main() {
    use self::schema::todos::dsl::{todos, completed};

    let id = args()
        .nth(1)
        .expect("complete_todo requires a todo id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = &mut establish_connection();

    let todo = diesel::update(todos.find(id))
        .set(completed.eq(true))
        .returning(Todo::as_returning())
        .get_result(connection)
        .unwrap();
    println!("Completed todo {}", todo.title);
}