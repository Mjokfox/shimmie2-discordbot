use crate::models::shimmie_json::{Fields, ShimmieSectionTypes};
use serenity::all::{ChannelId, Http};
use std::sync::Arc;
use embed::comment_embed::comment_embed;

use super::embed;

pub async fn comment_handler(http: Arc<Http>,r#type: &str, fields: Fields ) {
    match std::env::var("channelID") {
        Ok(id) =>{
            let ch = ChannelId::new(id.parse::<u64>().unwrap());
            match r#type {
                ShimmieSectionTypes::CREATE => {
                    let message = comment_embed(fields.post_id.unwrap_or_default(), fields.username.unwrap_or_default(), fields.comment_id.unwrap_or_default(),fields.message.unwrap_or_default());
                    if let Err(why) = ch.send_message(http, message).await {
                        println!("Error sending message: {why:?}");
                    }
                },
                _ => ()
            }
        },
        Err(_) => {
            eprintln!("No channel id given");
        }
    };
}