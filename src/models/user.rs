use diesel::prelude::*;
use serde::{Deserialize, Serialize};


//     users (id) {
//         id -> Int4,
        // uname -> Varchar,
//         email -> Varchar,
//         validated -> Bool,
//         created_at -> Timestamp,
//         updated_at -> Timestamp,
//     }
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub uname: String,
    pub email: String,
    //pub passhash: String,
    pub validated: bool,
}

use crate::schema::users;

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub uname: &'a str,
    pub email: &'a str,
}
