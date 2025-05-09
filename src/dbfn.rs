use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::result::Error;
use std::env;
use crate::models::db::{NewPost, Post, Comment, NewComment};
pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

// pub fn establish_connection() -> SqliteConnection {
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     SqliteConnection::establish(&database_url)
//         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
// }

pub fn establish_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

// posts

pub fn get_message_from_post_id(conn: &mut SqliteConnection, id: &i32) -> Result<i64, Error> {
    use crate::schema::posts::dsl::*;

    posts.filter(post_id.eq(id))
        .select(message_id)
        .first(conn)
}

pub fn create_post(conn: &mut SqliteConnection, post_id: &i32, message_id: &i64) {
    
    use crate::schema::posts;

    let new_post = NewPost { post_id, message_id };

    if let Err(why) = diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn) 
    {
        println!("Post saving failed: {why:?}");
    }
}

pub fn delete_post(conn: &mut SqliteConnection, p_id: &i32) {
    
    use crate::schema::posts::dsl::*;

    if let Err(why) = diesel::delete(posts.filter(post_id.eq(p_id))).execute(conn)
    {
        println!("Error deleting post: {why:?}");
    }
}

// comments

pub fn get_message_from_comment_id(conn: &mut SqliteConnection, id: &i32) -> Result<i64, Error> {
    
    use crate::schema::comments::dsl::*;

    comments.filter(comment_id.eq(id))
        .select(message_id)
        .first(conn)
}

pub fn create_comment(conn: &mut SqliteConnection, comment_id: &i32, post_id: &i32, message_id: &i64) {
    
    use crate::schema::comments;

    let new_post = NewComment { comment_id, post_id, message_id };

    if let Err(why) = diesel::insert_into(comments::table)
        .values(&new_post)
        .returning(Comment::as_returning())
        .get_result(conn)
    {
        println!("Comment saving failed: {why:?}");
    }
}

pub fn delete_comment(conn: &mut SqliteConnection, c_id: &i32) {
    
    use crate::schema::comments::dsl::*;

    if let Err(why) = diesel::delete(comments.filter(comment_id.eq(c_id))).execute(conn)
    {
        println!("Error deleting post: {why:?}");
    }
}

pub fn get_comment_messages_from_post_id(conn: &mut SqliteConnection, p_id: &i32) -> Result<Vec<i64>, Error> {

    use crate::schema::comments::dsl::*;

    comments.filter(post_id.eq(p_id))
        .select(message_id)
        .load(conn)
}

pub fn delete_comments_with_post_id(conn: &mut SqliteConnection, p_id: &i32) {

    use crate::schema::comments::dsl::*;

    if let Err(why) = diesel::delete(comments.filter(post_id.eq(p_id))).execute(conn)
    {
        println!("Error deleting comments: {why:?}");
    }
}