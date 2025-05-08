use diesel::prelude::*;
use crate::schema::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Post {
    pub post_id: i32,
    pub message_id: i64
}


#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub post_id: &'a i32,
    pub message_id: &'a i64,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = comments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Comment {
    pub comment_id: i32,
    pub post_id: i32,
    pub message_id: i64
}

#[derive(Insertable)]
#[diesel(table_name = comments)]
pub struct NewComment<'a> {
    pub comment_id: &'a i32,
    pub post_id: &'a i32,
    pub message_id: &'a i64,
}