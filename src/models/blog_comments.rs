use diesel::prelude::*;
use serde::{Deserialize, Serialize};


//     blog_posts (id) {
//         id -> Int4,
//         user_id -> Int4,
//         title -> Varchar,
//         body -> Text,
//         published -> Bool,
//         created_at -> Timestamp,
//         updated_at -> Timestamp,
//     }
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::blog_posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

use crate::schema::blog_posts;

#[derive(Insertable)]
#[diesel(table_name = blog_posts)]
pub struct NewPost<'a> {
    pub user_id: i32,
    pub title: &'a str,
    pub body: &'a str,
}
