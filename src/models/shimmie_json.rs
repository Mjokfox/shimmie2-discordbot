use serde::{Deserialize, Serialize};
use mime_serde_shim::Wrapper as Mime;

pub struct ShimmieSections{}

#[allow(dead_code)]
impl ShimmieSections {
    pub const COMMENT: &str = "comment";
    pub const IMAGE: &str = "image";
    pub const POST: &str = "image";
    pub const USER: &str = "user";
}

pub enum ShimmieSectionTypes{}

#[allow(dead_code)]
impl ShimmieSectionTypes {
    pub const CREATE: &str = "create";
    pub const EDIT: &str = "edit";
    pub const DELETE: &str = "delete";
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Fields {
    pub post_id: Option<i32>,
    pub username: Option<String>,
    pub hash: Option<String>,
    pub mime: Option<Mime>,
    pub size: Option<i32>,
    pub comment_id: Option<i32>,
    pub message: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShimmieJson<'a> {
    pub section: &'a str,
    pub r#type: &'a str,
    pub fields: Fields
}