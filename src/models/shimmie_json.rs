use mime_serde_shim::Wrapper as Mime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ShimmieSections {
    Comment,
    Image,
    User,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ShimmieSectionTypes {
    Create,
    Edit,
    Delete
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Fields {
    pub post_id: Option<i32>,
    pub username: Option<String>,
    pub hash: Option<String>,
    pub mime: Option<Mime>,
    pub size: Option<i32>,
    pub comment_id: Option<i32>,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShimmieJson {
    pub section: ShimmieSections,
    pub r#type: ShimmieSectionTypes,
    pub fields: Fields,
}