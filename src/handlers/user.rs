use std::sync::Arc;

use crate::models::shimmie_json::{Fields, HandlerTrait};
use serenity::all::{ChannelId, CreateEmbed, CreateMessage, Http, Timestamp};

pub struct UserHandler {
    pub http: Arc<Http>,
    pub ch: ChannelId,
    pub server_url: String
}

impl HandlerTrait for UserHandler {
    async fn create(&self, fields: Fields) {
        let embed = self.embed( 
            fields.username.unwrap_or_default()
        );
        let builder = CreateMessage::new().embed(embed);
        if let Err(why) = self.ch.send_message(self.http.clone(), builder).await {
            println!("Error sending user creation: {why:?}")
        }
    }

    async fn edit(&self, _fields: Fields) {}

    async fn delete(&self, fields: Fields) {
        if let Err(why) = self.ch.say(self.http.clone(), format!("User deleted: {}", fields.username.unwrap_or_default())).await {
            println!("Error sending user deletion: {why:?}")
        }
    }
}

impl UserHandler {
    fn embed(&self, username: String) -> CreateEmbed {
        CreateEmbed::new()
            .color(0xff8c00)
            .title(format!("New user: \"{}\"!", username))
            .url(format!("{}/user/{}",self.server_url, username))
            .timestamp(Timestamp::now())
    }
}