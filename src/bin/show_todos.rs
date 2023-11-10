use self::models::*;
use diesel::prelude::*;
use todo_api::*;

fn main() {
    use self::schema::todos::dsl::*;

    let connection = &mut establish_connection();
    let results = todos
        .filter(completed.eq(false))
        .limit(5)
        .select(Todo::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} todos", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}