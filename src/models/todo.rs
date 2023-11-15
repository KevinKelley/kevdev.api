use diesel::prelude::*;
use serde::{Deserialize, Serialize};


//     todos (id) {
//         id -> Int4,
//         title -> Varchar,
//         body -> Text,
//         completed -> Bool,
//         created_at -> Timestamp,
//         updated_at -> Timestamp,
//     }
#[derive(Queryable, Selectable, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub completed: bool,
}

use crate::schema::todos;

#[derive(Insertable)]
#[diesel(table_name = todos)]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
