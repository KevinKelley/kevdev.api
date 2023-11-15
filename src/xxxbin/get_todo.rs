use self::models::Todo;
use diesel::prelude::*;
use todo_api::*;
use std::env::args;

fn main() {
    use self::schema::todos::dsl::todos;

    let todo_id = args()
        .nth(1)
        .expect("get_todo requires a todo id")
        .parse::<i32>()
        .expect("Invalid ID");

    let connection = &mut establish_connection();

    let todo = todos
        .find(todo_id)
        .select(Todo::as_select())
        .first(connection)
        .optional(); // This allows for returning an Option<Todo>, otherwise it will throw an error

    match todo {
        Ok(Some(todo)) => println!("Todo with id: {} has a title: {}", todo.id, todo.title),
        Ok(None) => println!("Unable to find todo {}", todo_id),
        Err(_) => println!("An error occured while fetching todo {}", todo_id),
    }
}