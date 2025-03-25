use crate::models::shimmie_json::{Fields, ShimmieSectionTypes};
use serenity::all::{ChannelId, Http};
use std::sync::Arc;
use embed::image_embed::image_embed;
use super::embed;

pub async fn image_handler(http: Arc<Http>,r#type: &str, fields: Fields ) {
    match std::env::var("channelID") {
        Ok(id) =>{
            match r#type {
                ShimmieSectionTypes::CREATE => {
                    let ch = ChannelId::new(id.parse::<u64>().unwrap());
                    let message = image_embed(
                        fields.post_id.unwrap_or_default(), 
                        fields.username.unwrap_or_default(), 
                        fields.hash.unwrap_or_default(), 
                        fields.mime.unwrap_or(mime_serde_shim::Wrapper(mime::IMAGE_JPEG)).into(), 
                        fields.size.unwrap_or_default()
                    );
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