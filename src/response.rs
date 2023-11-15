use serde::Serialize;
use crate::models::todo::Todo;

//////////////////////////////
/// these for transport (as json)
/// 

#[derive(Serialize, Debug)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}


#[derive(Serialize, Debug)]
pub struct TodoData {
    pub todo: Todo,
}


#[derive(Serialize, Debug)]
pub struct SingleTodoResponse {
    pub status: String,
    pub data: TodoData,
}


#[derive(Serialize, Debug)]
pub struct TodoListResponse {
    pub status: String,
    pub results: usize,
    pub todos: Vec<Todo>,
}

