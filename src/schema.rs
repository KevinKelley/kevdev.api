// @generated automatically by Diesel CLI.

diesel::table! {
    blog_comments (id) {
        id -> Int4,
        user_id -> Int4,
        post_id -> Int4,
        body -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    blog_posts (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    todos (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        completed -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        uname -> Varchar,
        email -> Varchar,
        validated -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(blog_comments -> blog_posts (post_id));
diesel::joinable!(blog_comments -> users (user_id));
diesel::joinable!(blog_posts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    blog_comments,
    blog_posts,
    todos,
    users,
);
