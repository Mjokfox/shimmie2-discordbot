use mime_serde_shim::Wrapper as Mime;
use serde::{Deserialize, Serialize};
use crate::handlers::{comment::CommentHandler, image::ImageHandler, logging::LoggingHandler, user::UserHandler};

pub trait HandlerTrait {
    fn create(&self, fields: Fields) -> impl std::future::Future<Output = ()> + Send;
    fn edit(&self, fields: Fields) -> impl std::future::Future<Output = ()> + Send;
    fn delete(&self, fields: Fields) -> impl std::future::Future<Output = ()> + Send;
}

pub enum HandlerEnum {
    Comment(CommentHandler),
    Post(ImageHandler),
    User(UserHandler),
    Log(LoggingHandler),
}

impl HandlerTrait for HandlerEnum {
    async fn create(&self, fields: Fields) {
        match self {
            HandlerEnum::Comment(h) => h.create(fields).await,
            HandlerEnum::Post(h) => h.create(fields).await,
            HandlerEnum::User(h) => h.create(fields).await,
            HandlerEnum::Log(h) => h.create(fields).await,
        }
    }

    async fn edit(&self, fields: Fields) {
        match self {
            HandlerEnum::Comment(h) => h.edit(fields).await,
            HandlerEnum::Post(h) => h.edit(fields).await,
            HandlerEnum::User(h) => h.edit(fields).await,
            HandlerEnum::Log(h) => h.edit(fields).await,
        }
    }

    async fn delete(&self, fields: Fields) {
        match self {
            HandlerEnum::Comment(h) => h.delete(fields).await,
            HandlerEnum::Post(h) => h.delete(fields).await,
            HandlerEnum::User(h) => h.delete(fields).await,
            HandlerEnum::Log(h) => h.delete(fields).await,
        }
    }
}



#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ShimmieSections {
    Comment,
    Post,
    User,
    Log
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ShimmieSectionTypes {
    Create,
    Edit,
    Delete
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Fields {
    pub post_id: Option<i32>,
    pub username: Option<String>,
    pub hash: Option<String>,
    pub mime: Option<Mime>,
    pub size: Option<i32>,
    pub comment_id: Option<i32>,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ShimmieJson {
    pub section: ShimmieSections,
    pub r#type: ShimmieSectionTypes,
    pub fields: Fields,
}