// use diesel::prelude::*;

use axum::{
    routing::{get, post, patch, delete},
    Router,
};

use crate::{
    handler::{
        create_todo_handler, delete_todo_handler, edit_todo_handler, get_todo_handler,
        health_checker_handler, todos_list_handler,
    },
    models,
};

pub fn create_router() -> Router {
    // let db = model::todo_db();
    // let db_conn = crate::db::connection();
    // let db_conn = crate::db::todo_db();

    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api/todos", post(create_todo_handler))
        .route("/api/todos", get(todos_list_handler))
        .route("/api/todos/:id", get(get_todo_handler))
        .route("/api/todos/:id", patch(edit_todo_handler))
        .route("/api/todos/:id", delete(delete_todo_handler))
        // .with_state(db_conn)
}


// pub fn create_router() -> Router {
//     let db = model::todo_db();

//     Router::new()
//         .route("/api/healthchecker", get(health_checker_handler))
//         .route(
//             "/api/todos",
//             post(create_todo_handler).get(todos_list_handler),
//         )
//         .route(
//             "/api/todos/:id",
//             get(get_todo_handler)
//                 .patch(edit_todo_handler)
//                 .delete(delete_todo_handler),
//         )
//         .with_state(db)
// }

