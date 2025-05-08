use mime_serde_shim::Wrapper as Mime;
use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Debug)]
pub enum ShimmieSections {
    COMMENT,
    IMAGE,
    USER,
}

impl TryFrom<&str> for ShimmieSections {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "comment" => ShimmieSections::COMMENT,
            "image" => ShimmieSections::IMAGE,
            "user" => ShimmieSections::USER,
            _ => {
                return Err(());
            }
        })
    }
}

impl From<&ShimmieSections> for &str {
    fn from(val: &ShimmieSections) -> Self {
        match val {
            ShimmieSections::COMMENT => "comment",
            ShimmieSections::IMAGE => "image",
            ShimmieSections::USER => "user",
        }
    }
}

impl Serialize for ShimmieSections {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ShimmieSections {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
            deserializer.deserialize_str(StrVisitor)
    }
}

struct StrVisitor;

impl<'de> Visitor<'de> for StrVisitor {
    type Value = ShimmieSections;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("comment, image, user")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        TryInto::<ShimmieSections>::try_into(v).map_err(|_| E::custom(format!("nuh uh")))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        self.visit_str(v.as_ref())
    }
}

pub enum ShimmieSectionTypes {}

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
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShimmieJson<'a> {
    pub section: ShimmieSections,
    pub r#type: &'a str,
    pub fields: Fields,
}