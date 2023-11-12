use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::{
    models::{Todo, NewTodo},
    response::{SingleTodoResponse, TodoData, TodoListResponse},
};


#[derive(Debug, Deserialize, Default)]
pub struct QueryOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateTodoSchema {
    pub title: Option<String>,
    pub body: Option<String>,
    pub completed: Option<bool>,
}


// use std::sync::Arc;
// use tokio::sync::Mutex;
// pub type DB = Arc<Mutex<Vec<Todo>>>;

// pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;
pub type DB = crate::db::DbConnection;

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Build Simple CRUD API in Rust using Axum";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

////////////////////////////////////////////////////////////
/// HANDLERS
/// 

pub async fn todos_list_handler(
    opts: Option<Query<QueryOptions>>,
    State(mut db): State<DB>,
) -> impl IntoResponse {

    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let result = crate::read_all_todo(&mut db, offset as u32, limit as u32);

    let json_response = TodoListResponse {
        status: "success".to_string(),
        results: result.len(),
        todos: result,
    };

    Json(json_response)
}

pub async fn create_todo_handler(
    State(mut db): State<DB>,
    Json(mut data): Json<Todo>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    let todo = crate::find_todo(&mut db, &data.title);
    //let mut vec = db.lock().await;
    if let Some(todo) = todo    
    {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Todo with title: '{}' already exists", todo.title),
        });
        return Err((StatusCode::CONFLICT, Json(error_response)));
    }

    //let mut title = data.title.trim_end();
    //let mut body = data.body;

    // let uuid_id = Uuid::new_v4();
    // let datetime = chrono::Utc::now();

    // //data.id = Some(uuid_id.to_string());
    // data.completed = false;
    // // data.createdAt = Some(datetime);
    // // data.updatedAt = Some(datetime);

    let todo = crate::create_todo(&mut db, &data.title.trim_end(), &data.body);

    let json_response = SingleTodoResponse {
        status: "success".to_string(),
        data: TodoData { todo },
    };

    Ok((StatusCode::CREATED, Json(json_response)))
}

pub async fn get_todo_handler(
    Path(id): Path<i32>,
    State(mut db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let todo = crate::read_todo(&mut db, id);

    match todo {
        Ok(Some(todo)) => {
            // println!("Todo with id: {} has a title: {}", todo.id, todo.title)
            let json_response = SingleTodoResponse {
                status: "success".to_string(),
                data: TodoData { todo: todo.clone() },
            };
            Ok((StatusCode::OK, Json(json_response)))
        },
        Ok(None) => { 
            // println!("Unable to find todo {}", id)
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Todo with ID: {} not found", id)
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        },
        Err(_) => {
            // println!("An error occured while fetching todo {}", id)
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("internal error while searching for todo {}", id)
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        },
    }
}

pub async fn edit_todo_handler(
    Path(tid): Path<i32>,
    State(mut db): State<DB>,
    Json(data): Json<UpdateTodoSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    //let id = id.to_string();
    
    //let mut vec = db.lock().await;

    // if let Some(todo) = vec.iter_mut().find(|todo| todo.id.to_string() == id) {
    //     let datetime = chrono::Utc::now();
    //     let title = body
    //         .title
    //         .to_owned()
    //         .unwrap_or_else(|| todo.title.to_owned());
    //     let content = body
    //         .body
    //         .to_owned()
    //         .unwrap_or_else(|| todo.body.to_owned());
    //     let completed = body.completed.unwrap_or(todo.completed);
    //     let payload = Todo {
    //         id: todo.id.to_owned(),
    //         title: if !title.is_empty() {
    //             title
    //         } else {
    //             todo.title.to_owned()
    //         },
    //         body: if !content.is_empty() {
    //             content
    //         } else {
    //             todo.body.to_owned()
    //         },
    //         completed: completed,
    //         // createdAt: todo.createdAt,
    //         // updatedAt: Some(datetime),
    //     };
    //     *todo = payload;

    //     let json_response = SingleTodoResponse {
    //         status: "success".to_string(),
    //         data: TodoData { todo: todo.clone() },
    //     };
    //     Ok((StatusCode::OK, Json(json_response)))
    // } else {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Todo with ID: {} not found", tid)
        });

        Err((StatusCode::NOT_FOUND, Json(error_response)))
    // }
}

pub async fn delete_todo_handler(
    Path(id): Path<i32>,
    State(mut db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // let id = id.to_string();
    // let mut vec = db.lock().await;

    // if let Some(pos) = vec.iter().position(|todo| todo.id.to_string() == id) {
    //     vec.remove(pos);
    //     return Ok((StatusCode::NO_CONTENT, Json("")));
    // }

    let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("Todo with ID: {} not found", id)
    });

    Err((StatusCode::NOT_FOUND, Json(error_response)))
}

