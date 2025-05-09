use crate::models::shimmie_json::{Fields, HandlerTrait};

pub struct LoggingHandler {
}

impl HandlerTrait for LoggingHandler {
    async fn create(&self, _fields: Fields) {}

    async fn edit(&self, _fields: Fields) {}

    async fn delete(&self, _fields: Fields) {}
}

impl LoggingHandler {

}