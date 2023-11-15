use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::{
    models::todo::{Todo, NewTodo},
    response::{SingleTodoResponse, TodoData, TodoListResponse},
    db::*,
};
// use crate::models::todo::*;


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

// const ok        : StatusCode = StatusCode::OK;
// const created   : StatusCode = StatusCode::CREATED;
// const conflict  : StatusCode = StatusCode::CONFLICT;
// const error     : StatusCode = StatusCode::INTERNAL_SERVER_ERROR;
// const not_found : StatusCode = StatusCode::NOT_FOUND;
// const no_content: StatusCode = StatusCode::NO_CONTENT;
// ok.canonical_reason();
// ok.is_informational();
// ok.is_success();
// ok.is_redirection();
// ok.is_client_error();
// ok.is_server_error();



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
//    State(mut db): State<DB>,
) -> impl IntoResponse {

    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let result = read_all_todo(/*&mut db,*/ offset as u32, limit as u32);

    let json_response = TodoListResponse {
        status: "success".to_string(),
        results: result.len(),
        todos: result,
    };

    Json(json_response)
}

pub async fn create_todo_handler(
//    State(mut db): State<DB>,
    Json(mut data): Json<Todo>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    // can't allow a duplicate title...
    let todo = find_todo(/*&mut db,*/ &data.title);
    match todo {
        Ok(Some(todo)) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Todo with title: '{}' already exists", todo.title),
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
            ()
        },
        _ => ()
    }

    //let mut title = data.title.trim_end();
    //let mut body = data.body;

    // let uuid_id = Uuid::new_v4();
    // let datetime = chrono::Utc::now();

    // //data.id = Some(uuid_id.to_string());
    // data.completed = false;
    // // data.createdAt = Some(datetime);
    // // data.updatedAt = Some(datetime);

    let todo = create_todo(/*&mut db,*/ &data.title.trim_end(), &data.body);

    let json_response = SingleTodoResponse {
        status: "success".to_string(),
        data: TodoData { todo },
    };

    Ok((StatusCode::CREATED, Json(json_response)))
}

pub async fn get_todo_handler(
    Path(id): Path<i32>,
//    State(mut db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let todo = read_todo(/*&mut db,*/ id);

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
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        },
    }
}

pub async fn edit_todo_handler(
    Path(tid): Path<i32>,
//    State(mut db): State<DB>,
    Json(data): Json<UpdateTodoSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
   
    let todo = read_todo(/*&mut db,*/ tid);

    match todo {
        Ok(Some(todo)) => {
            //new values: if None was passed as option, use old value
            let title = data
                .title
                .to_owned()
                .unwrap_or_else(|| todo.title.to_owned());
            let body = data
                .body
                .to_owned()
                .unwrap_or_else(|| todo.body.to_owned());
            let completed = data.completed.unwrap_or(todo.completed);
            
            let updated_todo = update_todo(/*&mut db,*/ tid, &title, &body, completed);
            match updated_todo {
                Ok(Some(todo)) => {
                    let json_response = SingleTodoResponse {
                        status: "success".to_string(),
                        data: TodoData { todo: todo.clone() },
                    };
                    return Ok((StatusCode::OK, Json(json_response)))
                },
                Ok(None) => {
                    // println!("Unable to find todo {}", id)
                    let error_response = serde_json::json!({
                        "status": "fail",
                        "message": format!("Todo with ID: {} not found", tid)
                    });
                    return Err((StatusCode::NOT_FOUND, Json(error_response)))
                },
                Err(e) => {
                    // println!("An error occured while fetching todo {}", id)
                    let error_response = serde_json::json!({
                        "status": "fail",
                        "message": format!("internal error while searching for todo {}", tid)
                    });
                    return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
                },
            }
        },
        Ok(None) => { 
            // println!("Unable to find todo {}", id)
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Todo with ID: {} not found", tid)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)))
        },
        Err(_) => {
            // println!("An error occured while fetching todo {}", id)
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("internal error while searching for todo {}", tid)
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        },
    }
}

pub async fn delete_todo_handler(
    Path(id): Path<i32>,
//    State(mut db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // let id = id.to_string();
    // let mut vec = db.lock().await;

    // if let Some(pos) = vec.iter().position(|todo| todo.id.to_string() == id) {
    //     vec.remove(pos);
    //     return Ok((StatusCode::NO_CONTENT, Json("")));
    // }
    let todo = delete_todo(/*&mut db,*/ id);

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
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        },
    }

}

